/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use std::error::Error;
use std::fmt::Write;
use std::rc::Rc;
use std::string::String;
use std::sync::Arc;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::kernel::Settings;
use crate::math::tackler_real;
use crate::model::TxnAccount;
use crate::model::{posting, Commodity, Posting, Posts, Transaction, Txns};
use crate::parser::txn_antlr::txnparser::{
    CodeContextAttrs, DateContextAll, DateContextAttrs, TxnContextAttrs, TxnsContextAttrs, *,
};
use antlr_rust::tree::{ParseTree, Tree};
use itertools::Itertools;
use rust_decimal::Decimal;
use tackler_api::location::GeoPoint;
use tackler_api::txn_header::{Tag, Tags, TxnHeader};
use tackler_api::txn_ts;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

fn error_on_line<'a, T>(ctx: &Rc<T>, msg: &str) -> String
where
    T: ParserRuleContext<'a>,
{
    format!("Error on line: {}; {}", ctx.start().get_line(), msg)
}

fn handle_date(
    date_ctx: Rc<DateContextAll>,
    settings: &Settings,
) -> Result<OffsetDateTime, Box<dyn Error>> {
    let zoned_timestamp: Result<OffsetDateTime, Box<dyn Error>> = match date_ctx.TS_TZ() {
        Some(ts_tz) => match OffsetDateTime::parse(&ts_tz.get_text(), &Rfc3339) {
            Ok(zoned_ts) => Ok(zoned_ts),
            Err(_) => {
                let msg = error_on_line(&date_ctx, "timestamp with timezone");
                Err(msg.into())
            }
        },
        None => match date_ctx.TS() {
            Some(local_ts) => {
                let format = format_description!(
                        "[year]-[month]-[day]T[hour]:[minute]:[second][optional [.[subsecond digits:1+]]]");
                match PrimitiveDateTime::parse(&local_ts.get_text(), &format) {
                    Ok(local_ts) => Ok(settings.get_offset_datetime(local_ts)?),
                    Err(_) => {
                        let msg = error_on_line(&date_ctx, "timestamp in local time");
                        Err(msg.into())
                    }
                }
            }
            None => {
                let format = format_description!("[year]-[month]-[day]");
                match date_ctx.DATE() {
                    Some(d_ctx) => match time::Date::parse(&d_ctx.get_text(), &format) {
                        Ok(date) => Ok(settings.get_offset_date(date)?),
                        Err(_) => {
                            let msg = error_on_line(&date_ctx, "plain date");
                            Err(msg.into())
                        }
                    },
                    None => {
                        let msg = error_on_line(
                            &date_ctx,
                            "Internal logic error with timestamp handling",
                        );
                        Err(msg.into())
                    }
                }
            }
        },
    };
    zoned_timestamp
}

fn context_to_string<'a, T>(ctx: Rc<T>) -> String
where
    T: Tree<'a>,
{
    let v: Vec<String> = ctx.get_children().map(|c| c.get_text()).collect();
    v.join("")
}

fn handle_tag_ctx(
    tag_ctx: Rc<TagContextAll>,
    settings: &mut Settings,
) -> Result<Arc<Tag>, Box<dyn Error>> {
    let tag = context_to_string(tag_ctx);
    settings.get_or_create_tag(&tag)
}

fn handle_tags_ctx(
    tags_ctx: Rc<TagsContextAll>,
    settings: &mut Settings,
) -> Result<Tags, Box<dyn Error>> {
    // Tags parse tree ctx:
    //   tagsCtx.tag  always
    //   tagsCtx.tags sometimes (when multiple tags, recursive)
    //
    // See TxnParser.g4: 'txn_meta_tags' and 'tags' rules

    let tag = handle_tag_ctx(tags_ctx.tag().unwrap(/*:ok: parser */), settings)?;

    match tags_ctx.tags() {
        None => Ok(vec![tag]),
        Some(tags_tags_ctx) => {
            let mut tags = handle_tags_ctx(tags_tags_ctx, settings)?;
            tags.push(tag);
            Ok(tags)
        }
    }
}

type TxnMeta = (Option<Uuid>, Option<GeoPoint>, Option<Tags>);
fn handle_meta(
    meta_ctx: Rc<Txn_metaContextAll>,
    settings: &mut Settings,
) -> Result<TxnMeta, Box<dyn Error>> {
    let uuid = match meta_ctx.txn_meta_uuid(0) {
        Some(uuid_ctx) => {
            let uuid_str = uuid_ctx.UUID_VALUE().unwrap(/*:ok: parser */).get_text();
            match Uuid::parse_str(&uuid_str) {
                Ok(uuid) => Some(uuid),
                Err(err) => {
                    let msg = format!("Invalid UUID: {err}");
                    return Err(msg.into());
                }
            }
        }
        None => None,
    };

    let geo = match meta_ctx.txn_meta_location(0) {
        Some(geo_ctx) => {
            let uri_ctx = &geo_ctx.geo_uri().unwrap(/*:ok: parser */);
            Some(GeoPoint::from(
                // there must be lat, lon at least
                (uri_ctx.lat().unwrap(/*:ok: parser */).get_text()).parse::<Decimal>()?,
                (uri_ctx.lon().unwrap(/*:ok: parser */).get_text()).parse::<Decimal>()?,
                match uri_ctx.alt() {
                    Some(alt_ctx) => Some((alt_ctx.get_text()).parse::<Decimal>()?),
                    None => None,
                },
            )?)
        }
        None => None,
    };

    let tags: Option<Tags> = match meta_ctx.txn_meta_tags(0) {
        Some(tags_ctx) => Some(handle_tags_ctx(
            tags_ctx.tags().unwrap(/*:ok: parser */),
            settings,
        )?),
        None => None,
    };

    if let Some(t) = &tags {
        if t.len() != t.iter().unique().count() {
            let msg = format!("txn tags contains duplicate tags: {t:?}");
            return Err(msg.into());
        }
    }

    Ok((uuid, geo, tags))
}

fn handle_account(
    account_ctx: Rc<AccountContextAll>,
    commodity: Arc<Commodity>,
    settings: &mut Settings,
) -> Result<TxnAccount, Box<dyn Error>> {
    let account = context_to_string(account_ctx);

    settings.get_or_create_txn_account(&account, commodity)
}

fn handle_amount(amount_ctx: Rc<AmountContextAll>) -> Result<Decimal, Box<dyn Error>> {
    match tackler_real::from_str(&amount_ctx.get_text()) {
        Ok(d) => Ok(d),
        Err(err) => {
            let msg = format!(
                "Invalid value [{}], error was: {}",
                &amount_ctx.get_text(),
                err
            );
            Err(msg.into())
        }
    }
}
type ValuePosition = (Decimal, Decimal, bool, Arc<Commodity>, Arc<Commodity>);

fn handle_value_position(
    posting_ctx: &Rc<PostingContextAll>,
    settings: &mut Settings,
) -> Result<ValuePosition, Box<dyn Error>> {
    let post_commodity = match posting_ctx.opt_unit() {
        Some(u) => {
            settings.get_or_create_commodity(Some(&u.unit().unwrap(/*:ok: parser */).get_text()))?
        }
        None => settings.get_or_create_commodity(None)?,
    };

    // if txnCommodity (e.g. closing position) is not set, then use
    // posting commodity as txnCommodity.
    let txn_commodity = match posting_ctx.opt_unit() {
        Some(u) => {
            match u.opt_position() {
                Some(pos) => {
                    match pos.closing_pos() {
                        Some(cp) => {
                            // Ok, we have position, so there must be closing position
                            // so, we have closing position, use its commodity
                            let val_pos_commodity = settings.get_or_create_commodity(Some(
                                &cp.unit().unwrap(/*:ok: parser */).get_text(),
                            ))?;
                            if post_commodity.name == val_pos_commodity.name {
                                let em = format!(
                                    "Both commodities are same for value position [{}]",
                                    val_pos_commodity.name
                                );
                                let msg = error_on_line(posting_ctx, &em);
                                return Err(msg.into());
                            }
                            val_pos_commodity
                        }
                        None => settings.get_or_create_commodity(None)?,
                    }
                }
                None => {
                    // no position, use original unit
                    settings.get_or_create_commodity(Some(
                        &u.unit().unwrap(/*:ok: parser */).get_text(),
                    ))?
                }
            }
        }
        None => settings.get_or_create_commodity(None)?,
    };

    let post_amount = handle_amount(posting_ctx.amount().unwrap(/*:ok: parser */))?;

    let txn_amount: (Decimal, bool) = match posting_ctx.opt_unit() {
        Some(u) => {
            match u.opt_position() {
                Some(pos) => {
                    if let Some(opening_pos) = pos.opt_opening_pos() {
                        let opening_price =
                            handle_amount(opening_pos.amount().unwrap(/*:ok: parser */))?;
                        if opening_price.is_sign_negative() {
                            let msg = error_on_line(posting_ctx, "Unit cost '{ ... }' is negative");
                            return Err(msg.into());
                        }
                    }
                    match pos.closing_pos() {
                        Some(cp) => {
                            // ok, we have closing position
                            match cp.AT() {
                                None => {
                                    // this is '=', e.g. total price
                                    let total_cost =
                                        handle_amount(cp.amount().unwrap(/*:ok: parser :*/))?;
                                    if (total_cost.is_sign_negative()
                                        && post_amount.is_sign_positive())
                                        || (post_amount.is_sign_negative()
                                            && total_cost.is_sign_positive())
                                    {
                                        let msg = error_on_line(posting_ctx, "Total cost '=' has different sign than primary posting value");
                                        return Err(msg.into());
                                    }
                                    (total_cost, true)
                                }
                                Some(_) => {
                                    // this is '@', e.g. unit price
                                    let unit_price =
                                        handle_amount(cp.amount().unwrap(/*:ok: parser */))?;
                                    if unit_price.is_sign_negative() {
                                        let msg = error_on_line(
                                            posting_ctx,
                                            "Unit price '@' is negative",
                                        );
                                        return Err(msg.into());
                                    }
                                    (post_amount * unit_price, false)
                                }
                            }
                        }
                        None => {
                            // plain value, no closing position
                            (post_amount, false)
                        }
                    }
                }
                None => {
                    // No position at all
                    (post_amount, false)
                }
            }
        }
        None => (post_amount, false),
    };

    Ok((
        post_amount,
        txn_amount.0,
        txn_amount.1,
        post_commodity,
        txn_commodity,
    ))
}

fn handle_raw_posting(
    posting_ctx: &Rc<PostingContextAll>,
    settings: &mut Settings,
) -> Result<Posting, Box<dyn Error>> {
    let val_pos = handle_value_position(posting_ctx, settings)?;

    let atn = handle_account(
        posting_ctx.account().unwrap(/*:ok: parser */),
        val_pos.3,
        settings,
    )?;
    let comment: Option<String> = posting_ctx
        .opt_comment()
        .map(|c| c.comment().unwrap(/*:test:*/).text().unwrap(/*:ok: parser */).get_text());

    Ok((
        post_amount,
        txn_amount.0,
        txn_amount.1,
        post_commodity,
        txn_commodity,
    ))

    Posting::from(atn, val_pos.0, val_pos.1, val_pos.2, val_pos.4, comment)
}

fn handle_txn(
    txn_ctx: &Rc<TxnContextAll>,
    settings: &mut Settings,
) -> Result<Transaction, Box<dyn Error>> {
    let date = handle_date(txn_ctx.date().unwrap(/*:ok: parser */), settings)?;
    let code = txn_ctx
        .code()
        .map(|c| String::from(c.code_value().unwrap(/*:ok: parser */).get_text().trim()));

    let desc = match txn_ctx.description() {
        None => {
            // No description at all
            None
        }
        Some(d_ctx) => {
            // Ok, there was description
            // There is always "text" rule/token with current grammar (e.g. it can't be null).

            // right-trim, there was quote on the left side ...
            let s = String::from(d_ctx.text().unwrap(/*:ok: parser */).get_text().trim_end());
            Some(s)
        }
    };

    let meta = match txn_ctx.txn_meta() {
        Some(meta_ctx) => handle_meta(meta_ctx, settings)?,
        None => (None, None, None),
    };
    let uuid = meta.0;
    let location = meta.1;
    let tags = meta.2;

    if settings.audit_mode && uuid.is_none() {
        let mut msg = "Audit mode is activated and there is a txn without UUID".to_string();
        let _ = write!(msg, "\n   txn date: {}", txn_ts::rfc_3339(date));
        let _ = write!(
            msg,
            "{}",
            code.map(|c| format!("\n   txn code: {c}"))
                .unwrap_or_default()
        );
        return Err(msg.into());
    }

    let comments = {
        // txnCtx.txn_comment is never null, even when there aren't any comments
        // (in that case it will be an empty list)
        let cmts: Vec<String> = txn_ctx
            .txn_comment_all()
            .iter()
            .map(
                |c| c.comment().unwrap(/*:ok: parser */).text().unwrap(/*:ok: parser */).get_text(),
            )
            .collect();
        if cmts.is_empty() {
            None
        } else {
            Some(cmts)
        }
    };

    let posts_res: Result<Posts, Box<dyn Error>> = txn_ctx
        .postings()
        .unwrap(/*:ok: parser */)
        .posting_all()
        .iter()
        .map(|ctx| handle_raw_posting(ctx, settings))
        .collect();

    let mut posts = match posts_res {
        Ok(res) => res,
        Err(err) => {
            let msg = format!("{err}"); // todo: better error message, or remove this?
            return Err(msg.into());
        }
    };

    if posts.iter().map(|p| &p.txn_commodity.name).unique().count() > 1 {
        let msg = format!(
        "Different commodities without value positions are not allowed inside single transaction.{}", uuid.map(|u| format!("\n   txn uuid: {u}")).unwrap_or_default());
        return Err(msg.into());
    }

    let last_posting = match txn_ctx.postings().unwrap(/*:ok: parser */).last_posting() {
        Some(lp) => {
            let atn = handle_account(
                lp.account().unwrap(/*:ok: parser */),
                posts[0].txn_commodity.clone(),
                settings,
            )?;
            let amount = posting::txn_sum(&posts);
            let comment = lp.opt_comment().map(
                |c| c.comment().unwrap(/*:ok: parser */).text().unwrap(/*:ok: parser */).get_text(),
            );

            Some(Posting::from(
                atn,
                -amount,
                -amount,
                false,
                posts[0].txn_commodity.clone(),
                comment,
            )?)
        }
        None => None,
    };

    if let Some(lp) = last_posting {
        posts.push(lp)
    }

    Transaction::from(
        TxnHeader {
            timestamp: date,
            code,
            description: desc,
            uuid,
            location,
            tags,
            comments,
        },
        posts,
    )
}

pub(crate) fn handle_txns(
    txns_ctx: Rc<TxnsContextAll>,
    settings: &mut Settings,
) -> Result<Txns, Box<dyn Error>> {
    let txns = txns_ctx
        .txn_all()
        .iter()
        .map(|ctx| handle_txn(ctx, settings))
        .collect::<Result<Txns, Box<dyn Error>>>();

    txns
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;

    #[test]
    fn decimal_sign_logic() {
        assert!(Decimal::new(-1, 0).is_sign_negative());
        assert!(Decimal::new(0, 0).is_sign_positive());
        assert!(Decimal::new(1, 0).is_sign_positive());
    }
}
