#![allow(nonstandard_style)]
// Generated from TxnParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::txnparser::*;

pub trait TxnParserListener<'input> : ParseTreeListener<'input,TxnParserContextType>{
/**
 * Enter a parse tree produced by {@link TxnParser#txns}.
 * @param ctx the parse tree
 */
fn enter_txns(&mut self, _ctx: &TxnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txns}.
 * @param ctx the parse tree
 */
fn exit_txns(&mut self, _ctx: &TxnsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn}.
 * @param ctx the parse tree
 */
fn enter_txn(&mut self, _ctx: &TxnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn}.
 * @param ctx the parse tree
 */
fn exit_txn(&mut self, _ctx: &TxnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#date}.
 * @param ctx the parse tree
 */
fn enter_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#date}.
 * @param ctx the parse tree
 */
fn exit_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#code}.
 * @param ctx the parse tree
 */
fn enter_code(&mut self, _ctx: &CodeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#code}.
 * @param ctx the parse tree
 */
fn exit_code(&mut self, _ctx: &CodeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#code_value}.
 * @param ctx the parse tree
 */
fn enter_code_value(&mut self, _ctx: &Code_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#code_value}.
 * @param ctx the parse tree
 */
fn exit_code_value(&mut self, _ctx: &Code_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#description}.
 * @param ctx the parse tree
 */
fn enter_description(&mut self, _ctx: &DescriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#description}.
 * @param ctx the parse tree
 */
fn exit_description(&mut self, _ctx: &DescriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#text}.
 * @param ctx the parse tree
 */
fn enter_text(&mut self, _ctx: &TextContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#text}.
 * @param ctx the parse tree
 */
fn exit_text(&mut self, _ctx: &TextContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn_meta}.
 * @param ctx the parse tree
 */
fn enter_txn_meta(&mut self, _ctx: &Txn_metaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn_meta}.
 * @param ctx the parse tree
 */
fn exit_txn_meta(&mut self, _ctx: &Txn_metaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn_meta_uuid}.
 * @param ctx the parse tree
 */
fn enter_txn_meta_uuid(&mut self, _ctx: &Txn_meta_uuidContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn_meta_uuid}.
 * @param ctx the parse tree
 */
fn exit_txn_meta_uuid(&mut self, _ctx: &Txn_meta_uuidContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn_meta_location}.
 * @param ctx the parse tree
 */
fn enter_txn_meta_location(&mut self, _ctx: &Txn_meta_locationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn_meta_location}.
 * @param ctx the parse tree
 */
fn exit_txn_meta_location(&mut self, _ctx: &Txn_meta_locationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn_meta_tags}.
 * @param ctx the parse tree
 */
fn enter_txn_meta_tags(&mut self, _ctx: &Txn_meta_tagsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn_meta_tags}.
 * @param ctx the parse tree
 */
fn exit_txn_meta_tags(&mut self, _ctx: &Txn_meta_tagsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#geo_uri}.
 * @param ctx the parse tree
 */
fn enter_geo_uri(&mut self, _ctx: &Geo_uriContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#geo_uri}.
 * @param ctx the parse tree
 */
fn exit_geo_uri(&mut self, _ctx: &Geo_uriContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#lat}.
 * @param ctx the parse tree
 */
fn enter_lat(&mut self, _ctx: &LatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#lat}.
 * @param ctx the parse tree
 */
fn exit_lat(&mut self, _ctx: &LatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#lon}.
 * @param ctx the parse tree
 */
fn enter_lon(&mut self, _ctx: &LonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#lon}.
 * @param ctx the parse tree
 */
fn exit_lon(&mut self, _ctx: &LonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#alt}.
 * @param ctx the parse tree
 */
fn enter_alt(&mut self, _ctx: &AltContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#alt}.
 * @param ctx the parse tree
 */
fn exit_alt(&mut self, _ctx: &AltContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#tags}.
 * @param ctx the parse tree
 */
fn enter_tags(&mut self, _ctx: &TagsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#tags}.
 * @param ctx the parse tree
 */
fn exit_tags(&mut self, _ctx: &TagsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#tag}.
 * @param ctx the parse tree
 */
fn enter_tag(&mut self, _ctx: &TagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#tag}.
 * @param ctx the parse tree
 */
fn exit_tag(&mut self, _ctx: &TagContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#txn_comment}.
 * @param ctx the parse tree
 */
fn enter_txn_comment(&mut self, _ctx: &Txn_commentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#txn_comment}.
 * @param ctx the parse tree
 */
fn exit_txn_comment(&mut self, _ctx: &Txn_commentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#indent}.
 * @param ctx the parse tree
 */
fn enter_indent(&mut self, _ctx: &IndentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#indent}.
 * @param ctx the parse tree
 */
fn exit_indent(&mut self, _ctx: &IndentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#comment}.
 * @param ctx the parse tree
 */
fn enter_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#comment}.
 * @param ctx the parse tree
 */
fn exit_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#postings}.
 * @param ctx the parse tree
 */
fn enter_postings(&mut self, _ctx: &PostingsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#postings}.
 * @param ctx the parse tree
 */
fn exit_postings(&mut self, _ctx: &PostingsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#posting}.
 * @param ctx the parse tree
 */
fn enter_posting(&mut self, _ctx: &PostingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#posting}.
 * @param ctx the parse tree
 */
fn exit_posting(&mut self, _ctx: &PostingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#last_posting}.
 * @param ctx the parse tree
 */
fn enter_last_posting(&mut self, _ctx: &Last_postingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#last_posting}.
 * @param ctx the parse tree
 */
fn exit_last_posting(&mut self, _ctx: &Last_postingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#opt_unit}.
 * @param ctx the parse tree
 */
fn enter_opt_unit(&mut self, _ctx: &Opt_unitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#opt_unit}.
 * @param ctx the parse tree
 */
fn exit_opt_unit(&mut self, _ctx: &Opt_unitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#opt_comment}.
 * @param ctx the parse tree
 */
fn enter_opt_comment(&mut self, _ctx: &Opt_commentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#opt_comment}.
 * @param ctx the parse tree
 */
fn exit_opt_comment(&mut self, _ctx: &Opt_commentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#opt_position}.
 * @param ctx the parse tree
 */
fn enter_opt_position(&mut self, _ctx: &Opt_positionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#opt_position}.
 * @param ctx the parse tree
 */
fn exit_opt_position(&mut self, _ctx: &Opt_positionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#opt_opening_pos}.
 * @param ctx the parse tree
 */
fn enter_opt_opening_pos(&mut self, _ctx: &Opt_opening_posContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#opt_opening_pos}.
 * @param ctx the parse tree
 */
fn exit_opt_opening_pos(&mut self, _ctx: &Opt_opening_posContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#closing_pos}.
 * @param ctx the parse tree
 */
fn enter_closing_pos(&mut self, _ctx: &Closing_posContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#closing_pos}.
 * @param ctx the parse tree
 */
fn exit_closing_pos(&mut self, _ctx: &Closing_posContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#account}.
 * @param ctx the parse tree
 */
fn enter_account(&mut self, _ctx: &AccountContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#account}.
 * @param ctx the parse tree
 */
fn exit_account(&mut self, _ctx: &AccountContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#amount}.
 * @param ctx the parse tree
 */
fn enter_amount(&mut self, _ctx: &AmountContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#amount}.
 * @param ctx the parse tree
 */
fn exit_amount(&mut self, _ctx: &AmountContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#unit}.
 * @param ctx the parse tree
 */
fn enter_unit(&mut self, _ctx: &UnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#unit}.
 * @param ctx the parse tree
 */
fn exit_unit(&mut self, _ctx: &UnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#sp}.
 * @param ctx the parse tree
 */
fn enter_sp(&mut self, _ctx: &SpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#sp}.
 * @param ctx the parse tree
 */
fn exit_sp(&mut self, _ctx: &SpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#opt_sp}.
 * @param ctx the parse tree
 */
fn enter_opt_sp(&mut self, _ctx: &Opt_spContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#opt_sp}.
 * @param ctx the parse tree
 */
fn exit_opt_sp(&mut self, _ctx: &Opt_spContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TxnParser#blankline}.
 * @param ctx the parse tree
 */
fn enter_blankline(&mut self, _ctx: &BlanklineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TxnParser#blankline}.
 * @param ctx the parse tree
 */
fn exit_blankline(&mut self, _ctx: &BlanklineContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : TxnParserListener<'input> }


