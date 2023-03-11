/*
 * Copyright 2023 E257.FI
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use std::error::Error;
use std::rc::Rc;
use std::string::String;
use uuid::Uuid;

use crate::math::tackler_real;
use crate::model::{posting, AccountTreeNode, Commodity, Posting, Posts, Transaction, Txns};
use crate::parser::txn_antlr::txnparser::{
    CodeContextAttrs, DateContextAll, DateContextAttrs, TxnContextAttrs, TxnsContextAttrs, *,
};
use antlr_rust::tree::{ParseTree, Tree};
use itertools::Itertools;
use rust_decimal::Decimal;
use tackler_api::{GeoPoint, Tag, Tags, TxnHeader};

fn error_on_line<'a, T>(ctx: &Rc<T>, msg: &str) -> String
where
    T: ParserRuleContext<'a>,
{
    format!("Error on line: {}; {}", ctx.start().get_line(), msg)
}

fn handle_date(date_ctx: Rc<DateContextAll>) -> Result<DateTime<FixedOffset>, Box<dyn Error>> {
    let zoned_timestamp: Result<DateTime<FixedOffset>, Box<dyn Error>> = match date_ctx.TS_TZ() {
        Some(ts_tz) => {
            match ts_tz.get_text().parse::<DateTime<FixedOffset>>() {
                Ok(zoned_ts) => Ok(zoned_ts),
                Err(_) => {
                    // todo: err
                    let msg = error_on_line(&date_ctx, "timezone");
                    Err(msg.into())
                }
            }
        }
        None => {
            match date_ctx.TS() {
                Some(local_ts) => {
                    match local_ts.get_text().parse::<NaiveDateTime>() {
                        Ok(local_ts) => {
                            // todo: fix zone by cfg
                            Ok(DateTime::from_local(
                                local_ts,
                                FixedOffset::east_opt(0).unwrap(),
                            ))
                        }
                        Err(_) => {
                            // todo: err
                            Err("todo".into())
                        }
                    }
                }
                None => {
                    match date_ctx.DATE() {
                        Some(d_ctx) => match d_ctx.get_text().parse::<NaiveDate>() {
                            Ok(date) => {
                                // todo: fix time by cfg
                                let naive_ts = NaiveDateTime::new(
                                    date,
                                    NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                                );
                                // todo: fix zone by cfg
                                Ok(DateTime::from_local(
                                    naive_ts,
                                    FixedOffset::east_opt(0).unwrap(),
                                ))
                            }
                            Err(_) => {
                                // todo: err
                                Err("todo".into())
                            }
                        },
                        None => {
                            // todo: intrenal error
                            Err("todo".into())
                        }
                    }
                }
            }
        }
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

fn handle_tag_ctx(tag_ctx: Rc<TagContextAll>) -> Result<Tag, Box<dyn Error>> {
    let tag = context_to_string(tag_ctx);
    // todo: cfg.strict check chart of tags
    Ok(tag)
}

fn handle_tags_ctx(tags_ctx: Rc<TagsContextAll>) -> Result<Tags, Box<dyn Error>> {
    // Tags parse tree ctx:
    //   tagsCtx.tag  always
    //   tagsCtx.tags sometimes (when multiple tags, recursive)
    //
    // See TxnParser.g4: 'txn_meta_tags' and 'tags' rules

    let tag = handle_tag_ctx(tags_ctx.tag().unwrap())?;

    match tags_ctx.tags() {
        None => Ok(vec![tag]),
        Some(tags_tags_ctx) => {
            let mut tags = handle_tags_ctx(tags_tags_ctx)?;
            tags.push(tag);
            Ok(tags)
        }
    }
}

fn handle_meta(
    meta_ctx: Rc<Txn_metaContextAll>,
) -> Result<(Option<Uuid>, Option<GeoPoint>, Option<Tags>), Box<dyn Error>> {
    let uuid = match meta_ctx.txn_meta_uuid(0) {
        Some(uuid_ctx) => {
            let uuid_str = uuid_ctx.UUID_VALUE().unwrap().get_text();
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
            let uri_ctx = &geo_ctx.geo_uri().unwrap();
            Some(GeoPoint::from(
                // there must be lat, lon at least
                (uri_ctx.lat().unwrap().get_text()).parse::<f64>()?,
                (uri_ctx.lon().unwrap().get_text()).parse::<f64>()?,
                match uri_ctx.alt() {
                    Some(alt_ctx) => Some((alt_ctx.get_text()).parse::<f64>()?),
                    None => None,
                },
            )?)
        }
        None => None,
    };

    let tags: Option<Tags> = match meta_ctx.txn_meta_tags(0) {
        Some(tags_ctx) => Some(handle_tags_ctx(tags_ctx.tags().unwrap())?),
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
    commodity: Option<Commodity>,
) -> Result<AccountTreeNode, Box<dyn Error>> {
    let account = context_to_string(account_ctx);

    // todo: check cfg.strict => account is defined

    AccountTreeNode::from(account, commodity)
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

fn handle_value_position(
    posting_ctx: &Rc<PostingContextAll>,
) -> Result<(Decimal, Decimal, bool, Option<Commodity>, Option<Commodity>), Box<dyn Error>> {
    let post_commodity = posting_ctx
        .opt_unit()
        .map(|u| Commodity::from(u.unit().unwrap().get_text()).unwrap());

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
                            let val_pos_commodity =
                                Commodity::from(cp.unit().unwrap().get_text()).unwrap();
                            if let Some(p) = &post_commodity {
                                if p.name == val_pos_commodity.name {
                                    let em = format!(
                                        "Both commodities are same for value position [{}]",
                                        val_pos_commodity.name
                                    );
                                    let msg = error_on_line(posting_ctx, &em);
                                    return Err(msg.into());
                                }
                            }
                            Some(val_pos_commodity)
                        }
                        None => None,
                    }
                }
                None => {
                    // no position, use original unit
                    Some(Commodity::from(u.unit().unwrap().get_text()).unwrap())
                }
            }
        }
        None => None,
    };

    let post_amount = handle_amount(posting_ctx.amount().unwrap())?;

    let txn_amount: (Decimal, bool) = match posting_ctx.opt_unit() {
        Some(u) => {
            match u.opt_position() {
                Some(pos) => {
                    if let Some(opening_pos) = pos.opt_opening_pos() {
                        let opening_price = handle_amount(opening_pos.amount().unwrap())?;
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
                                    let total_cost = handle_amount(cp.amount().unwrap())?;
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
                                    let unit_price = handle_amount(cp.amount().unwrap())?;
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

fn handle_raw_posting(posting_ctx: &Rc<PostingContextAll>) -> Result<Posting, Box<dyn Error>> {
    let foo = handle_value_position(posting_ctx)?;

    /*
    // todo: check & Error
    if (settings.Accounts.strict) {
        checkCommodity(foo._4, postingCtx)
        checkCommodity(foo._5, postingCtx)
    }
    */

    let atn = handle_account(posting_ctx.account().unwrap(), foo.3)?;
    let comment: Option<String> = posting_ctx
        .opt_comment()
        .map(|c| c.comment().unwrap().text().unwrap().get_text());

    Posting::from(atn, foo.0, foo.1, foo.2, foo.4, comment)
}

fn handle_txn(txn_ctx: &Rc<TxnContextAll>) -> Result<Transaction, Box<dyn Error>> {
    let date = handle_date(txn_ctx.date().unwrap())?;
    let code = txn_ctx
        .code()
        .map(|c| String::from(c.code_value().unwrap().get_text().trim()));

    let desc = match txn_ctx.description() {
        None => {
            // No description at all
            None
        }
        Some(d_ctx) => {
            // Ok, there was description
            // There is always "text" rule/token with current grammar (e.g. it can't be null).

            // right-trim, there was quote on the left side ...
            let s = String::from(d_ctx.text().unwrap().get_text().trim_end());
            Some(s)
        }
    };

    let meta = match txn_ctx.txn_meta() {
        Some(meta_ctx) => handle_meta(meta_ctx)?,
        None => (None, None, None),
    };
    let uuid = meta.0;
    let location = meta.1;
    let tags = meta.2;

    // todo: check cfg.auditing && Uuid == None => error
    // "Configuration setting '" + CfgKeys.Auditing.txnSetChecksum + "' is activated and there is txn without UUID."

    let comments = {
        // txnCtx.txn_comment is never null, even when there aren't any comments
        // (in that case it will be an empty list)
        let cmts: Vec<String> = txn_ctx
            .txn_comment_all()
            .iter()
            .map(|c| c.comment().unwrap().text().unwrap().get_text())
            .collect();
        if cmts.is_empty() {
            None
        } else {
            Some(cmts)
        }
    };

    let posts_res: Result<Posts, Box<dyn Error>> = txn_ctx
        .postings()
        .unwrap()
        .posting_all()
        .iter()
        .map(handle_raw_posting)
        .collect();

    let mut posts = match posts_res {
        Ok(res) => res,
        Err(err) => {
            let msg = format!("{err}"); // todo: better error message, or remove this?
            return Err(msg.into());
        }
    };

    if posts
        .iter()
        .map(|p| match &p.txn_commodity {
            Some(c) => c.name.clone(),
            None => "".to_string(),
        })
        .unique()
        .count()
        > 1
    {
        let msg = format!(
        "Different commodities without value positions are not allowed inside single transaction.{}", uuid.map(|u| format!("\n   txn uuid: {u}")).unwrap_or_default());
        return Err(msg.into());
    }

    let last_posting = match txn_ctx.postings().unwrap().last_posting() {
        Some(lp) => {
            let atn = handle_account(lp.account().unwrap(), posts[0].txn_commodity.clone())?;
            let amount = posting::txn_sum(&posts);
            let comment = lp
                .opt_comment()
                .map(|c| c.comment().unwrap().text().unwrap().get_text());

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

pub(crate) fn handle_txns(txns_ctx: Rc<TxnsContextAll>) -> Result<Txns, Box<dyn Error>> {
    let txns = txns_ctx
        .txn_all()
        .iter()
        .map(handle_txn)
        .collect::<Result<Txns, Box<dyn Error>>>();

    txns
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;

    #[test]
    fn decimal_sign_logic() {
        assert_eq!(true, Decimal::new(-1, 0).is_sign_negative());
        assert_eq!(true, Decimal::new(0, 0).is_sign_positive());
        assert_eq!(true, Decimal::new(1, 0).is_sign_positive());
    }
}
