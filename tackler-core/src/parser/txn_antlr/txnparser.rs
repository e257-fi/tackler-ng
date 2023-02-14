// Generated from TxnParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::txnparserlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const UUID_NAME:isize=1; 
		pub const LOCATION_NAME:isize=2; 
		pub const GEO_NAME:isize=3; 
		pub const TAGS_NAME:isize=4; 
		pub const UUID_VALUE:isize=5; 
		pub const DATE:isize=6; 
		pub const TS:isize=7; 
		pub const TS_TZ:isize=8; 
		pub const INT:isize=9; 
		pub const NUMBER:isize=10; 
		pub const ID:isize=11; 
		pub const SUBID:isize=12; 
		pub const QUOTE:isize=13; 
		pub const L_BRACE:isize=14; 
		pub const R_BRACE:isize=15; 
		pub const L_CURLY:isize=16; 
		pub const R_CURLY:isize=17; 
		pub const L_SQUARE:isize=18; 
		pub const R_SQUARE:isize=19; 
		pub const L_ANGLE:isize=20; 
		pub const R_ANGLE:isize=21; 
		pub const HASH:isize=22; 
		pub const AT:isize=23; 
		pub const EQUAL:isize=24; 
		pub const SPACE:isize=25; 
		pub const TAB:isize=26; 
		pub const COMMA:isize=27; 
		pub const SEMICOLON:isize=28; 
		pub const COLON:isize=29; 
		pub const NL:isize=30; 
		pub const ANYCHAR:isize=31;
	pub const RULE_txns:usize = 0; 
	pub const RULE_txn:usize = 1; 
	pub const RULE_date:usize = 2; 
	pub const RULE_code:usize = 3; 
	pub const RULE_code_value:usize = 4; 
	pub const RULE_description:usize = 5; 
	pub const RULE_text:usize = 6; 
	pub const RULE_txn_meta:usize = 7; 
	pub const RULE_txn_meta_uuid:usize = 8; 
	pub const RULE_txn_meta_location:usize = 9; 
	pub const RULE_txn_meta_tags:usize = 10; 
	pub const RULE_geo_uri:usize = 11; 
	pub const RULE_lat:usize = 12; 
	pub const RULE_lon:usize = 13; 
	pub const RULE_alt:usize = 14; 
	pub const RULE_tags:usize = 15; 
	pub const RULE_tag:usize = 16; 
	pub const RULE_txn_comment:usize = 17; 
	pub const RULE_indent:usize = 18; 
	pub const RULE_comment:usize = 19; 
	pub const RULE_postings:usize = 20; 
	pub const RULE_posting:usize = 21; 
	pub const RULE_last_posting:usize = 22; 
	pub const RULE_opt_unit:usize = 23; 
	pub const RULE_opt_comment:usize = 24; 
	pub const RULE_opt_position:usize = 25; 
	pub const RULE_opt_opening_pos:usize = 26; 
	pub const RULE_closing_pos:usize = 27; 
	pub const RULE_account:usize = 28; 
	pub const RULE_amount:usize = 29; 
	pub const RULE_unit:usize = 30; 
	pub const RULE_sp:usize = 31; 
	pub const RULE_opt_sp:usize = 32; 
	pub const RULE_blankline:usize = 33;
	pub const ruleNames: [&'static str; 34] =  [
		"txns", "txn", "date", "code", "code_value", "description", "text", "txn_meta", 
		"txn_meta_uuid", "txn_meta_location", "txn_meta_tags", "geo_uri", "lat", 
		"lon", "alt", "tags", "tag", "txn_comment", "indent", "comment", "postings", 
		"posting", "last_posting", "opt_unit", "opt_comment", "opt_position", 
		"opt_opening_pos", "closing_pos", "account", "amount", "unit", "sp", "opt_sp", 
		"blankline"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;30] = [
		None, Some("'uuid'"), Some("'location'"), Some("'geo'"), Some("'tags'"), 
		None, None, None, None, None, None, None, None, Some("'''"), Some("'('"), 
		Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), Some("'<'"), 
		Some("'>'"), Some("'#'"), Some("'@'"), Some("'='"), Some("' '"), Some("'\t'"), 
		Some("','"), Some("';'"), Some("':'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;32]  = [
		None, Some("UUID_NAME"), Some("LOCATION_NAME"), Some("GEO_NAME"), Some("TAGS_NAME"), 
		Some("UUID_VALUE"), Some("DATE"), Some("TS"), Some("TS_TZ"), Some("INT"), 
		Some("NUMBER"), Some("ID"), Some("SUBID"), Some("QUOTE"), Some("L_BRACE"), 
		Some("R_BRACE"), Some("L_CURLY"), Some("R_CURLY"), Some("L_SQUARE"), Some("R_SQUARE"), 
		Some("L_ANGLE"), Some("R_ANGLE"), Some("HASH"), Some("AT"), Some("EQUAL"), 
		Some("SPACE"), Some("TAB"), Some("COMMA"), Some("SEMICOLON"), Some("COLON"), 
		Some("NL"), Some("ANYCHAR")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TxnParserExt<'input>, I, TxnParserContextType , dyn TxnParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TxnParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TxnParserContextType , dyn TxnParserListener<'input> + 'a>;

/// Parser for TxnParser grammar
pub struct TxnParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				TxnParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> TxnParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TxnParser<'input, I, DefaultErrorStrategy<'input,TxnParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TxnParser
pub trait TxnParserContext<'input>:
	for<'x> Listenable<dyn TxnParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TxnParserContextType>
{}

antlr_rust::coerce_from!{ 'input : TxnParserContext<'input> }

impl<'input> TxnParserContext<'input> for TerminalNode<'input,TxnParserContextType> {}
impl<'input> TxnParserContext<'input> for ErrorNode<'input,TxnParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TxnParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TxnParserListener<'input> + 'input }

pub struct TxnParserContextType;
antlr_rust::tid!{TxnParserContextType}

impl<'input> ParserNodeType<'input> for TxnParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TxnParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TxnParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> TxnParserExt<'input>{
}
antlr_rust::tid! { TxnParserExt<'a> }

impl<'input> TokenAware<'input> for TxnParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TxnParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TxnParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "TxnParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn TxnParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					7 => TxnParser::<'input,I,_>::txn_meta_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					15 => TxnParser::<'input,I,_>::tags_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> TxnParser<'input, I, DefaultErrorStrategy<'input,TxnParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn txn_meta_sempred(_localctx: Option<&Txn_metaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					let _localctx = _localctx.unwrap();
					*_localctx.get_u() < 1
				}
				1=>{
					let _localctx = _localctx.unwrap();
					*_localctx.get_l() < 1
				}
				2=>{
					let _localctx = _localctx.unwrap();
					*_localctx.get_t() < 1
				}
			_ => true
		}
	}
	fn tags_sempred(_localctx: Option<&TagsContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- txns ----------------
pub type TxnsContextAll<'input> = TxnsContext<'input>;


pub type TxnsContext<'input> = BaseParserRuleContext<'input,TxnsContextExt<'input>>;

#[derive(Clone)]
pub struct TxnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for TxnsContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for TxnsContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txns(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txns(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TxnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txns }
}
antlr_rust::tid!{TxnsContextExt<'a>}

impl<'input> TxnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TxnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TxnsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TxnsContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<TxnsContextExt<'input>>{

fn txn_all(&self) ->  Vec<Rc<TxnContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txn(&self, i: usize) -> Option<Rc<TxnContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn blankline_all(&self) ->  Vec<Rc<BlanklineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blankline(&self, i: usize) -> Option<Rc<BlanklineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TxnsContextAttrs<'input> for TxnsContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txns(&mut self,)
	-> Result<Rc<TxnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TxnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_txns);
        let mut _localctx: Rc<TxnsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(71);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << SPACE) | (1usize << TAB) | (1usize << NL))) != 0) {
				{
				{
				/*InvokeRule blankline*/
				recog.base.set_state(68);
				recog.blankline()?;

				}
				}
				recog.base.set_state(73);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule txn*/
			recog.base.set_state(74);
			recog.txn()?;

			recog.base.set_state(84);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(76); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule blankline*/
						recog.base.set_state(75);
						recog.blankline()?;

						}
						}
						recog.base.set_state(78); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << SPACE) | (1usize << TAB) | (1usize << NL))) != 0)) {break}
					}
					/*InvokeRule txn*/
					recog.base.set_state(80);
					recog.txn()?;

					}
					} 
				}
				recog.base.set_state(86);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			recog.base.set_state(90);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule blankline*/
					recog.base.set_state(87);
					recog.blankline()?;

					}
					} 
				}
				recog.base.set_state(92);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			/*InvokeRule opt_sp*/
			recog.base.set_state(93);
			recog.opt_sp()?;

			recog.base.set_state(94);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn ----------------
pub type TxnContextAll<'input> = TxnContext<'input>;


pub type TxnContext<'input> = BaseParserRuleContext<'input,TxnContextExt<'input>>;

#[derive(Clone)]
pub struct TxnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for TxnContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for TxnContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TxnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn }
}
antlr_rust::tid!{TxnContextExt<'a>}

impl<'input> TxnContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TxnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TxnContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TxnContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<TxnContextExt<'input>>{

fn date(&self) -> Option<Rc<DateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NL
/// Returns `None` if there is no child corresponding to token NL
fn NL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, 0)
}
fn postings(&self) -> Option<Rc<PostingsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn description(&self) -> Option<Rc<DescriptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn code(&self) -> Option<Rc<CodeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txn_meta(&self) -> Option<Rc<Txn_metaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txn_comment_all(&self) ->  Vec<Rc<Txn_commentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txn_comment(&self, i: usize) -> Option<Rc<Txn_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TxnContextAttrs<'input> for TxnContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn(&mut self,)
	-> Result<Rc<TxnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TxnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_txn);
        let mut _localctx: Rc<TxnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule date*/
			recog.base.set_state(96);
			recog.date()?;

			recog.base.set_state(98);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule code*/
					recog.base.set_state(97);
					recog.code()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule description*/
					recog.base.set_state(100);
					recog.description()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule opt_sp*/
					recog.base.set_state(101);
					recog.opt_sp()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(104);
			recog.base.match_token(NL,&mut recog.err_handler)?;

			recog.base.set_state(106);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule txn_meta*/
					recog.base.set_state(105);
					recog.txn_meta(0, 0, 0)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(111);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule txn_comment*/
					recog.base.set_state(108);
					recog.txn_comment()?;

					}
					} 
				}
				recog.base.set_state(113);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			}
			/*InvokeRule postings*/
			recog.base.set_state(114);
			recog.postings()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- date ----------------
pub type DateContextAll<'input> = DateContext<'input>;


pub type DateContext<'input> = BaseParserRuleContext<'input,DateContextExt<'input>>;

#[derive(Clone)]
pub struct DateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for DateContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for DateContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_date(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_date(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_date }
	//fn type_rule_index() -> usize where Self: Sized { RULE_date }
}
antlr_rust::tid!{DateContextExt<'a>}

impl<'input> DateContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DateContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<DateContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DATE
/// Returns `None` if there is no child corresponding to token DATE
fn DATE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(DATE, 0)
}
/// Retrieves first TerminalNode corresponding to token TS
/// Returns `None` if there is no child corresponding to token TS
fn TS(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TS, 0)
}
/// Retrieves first TerminalNode corresponding to token TS_TZ
/// Returns `None` if there is no child corresponding to token TS_TZ
fn TS_TZ(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TS_TZ, 0)
}

}

impl<'input> DateContextAttrs<'input> for DateContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn date(&mut self,)
	-> Result<Rc<DateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_date);
        let mut _localctx: Rc<DateContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(116);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << DATE) | (1usize << TS) | (1usize << TS_TZ))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- code ----------------
pub type CodeContextAll<'input> = CodeContext<'input>;


pub type CodeContext<'input> = BaseParserRuleContext<'input,CodeContextExt<'input>>;

#[derive(Clone)]
pub struct CodeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for CodeContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for CodeContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_code(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_code(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CodeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_code }
	//fn type_rule_index() -> usize where Self: Sized { RULE_code }
}
antlr_rust::tid!{CodeContextExt<'a>}

impl<'input> CodeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CodeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CodeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CodeContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<CodeContextExt<'input>>{

fn sp(&self) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn code_value(&self) -> Option<Rc<Code_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> CodeContextAttrs<'input> for CodeContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn code(&mut self,)
	-> Result<Rc<CodeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CodeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_code);
        let mut _localctx: Rc<CodeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule sp*/
			recog.base.set_state(118);
			recog.sp()?;

			recog.base.set_state(119);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule code_value*/
			recog.base.set_state(120);
			recog.code_value()?;

			recog.base.set_state(121);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- code_value ----------------
pub type Code_valueContextAll<'input> = Code_valueContext<'input>;


pub type Code_valueContext<'input> = BaseParserRuleContext<'input,Code_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Code_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Code_valueContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Code_valueContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_code_value(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_code_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Code_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_code_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_code_value }
}
antlr_rust::tid!{Code_valueContextExt<'a>}

impl<'input> Code_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Code_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Code_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Code_valueContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Code_valueContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token QUOTE in current rule
fn QUOTE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token QUOTE, starting from 0.
/// Returns `None` if number of children corresponding to token QUOTE is less or equal than `i`.
fn QUOTE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(QUOTE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token L_SQUARE in current rule
fn L_SQUARE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_SQUARE, starting from 0.
/// Returns `None` if number of children corresponding to token L_SQUARE is less or equal than `i`.
fn L_SQUARE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_SQUARE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_SQUARE in current rule
fn R_SQUARE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_SQUARE, starting from 0.
/// Returns `None` if number of children corresponding to token R_SQUARE is less or equal than `i`.
fn R_SQUARE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_SQUARE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token L_CURLY in current rule
fn L_CURLY_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_CURLY, starting from 0.
/// Returns `None` if number of children corresponding to token L_CURLY is less or equal than `i`.
fn L_CURLY(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_CURLY, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_CURLY in current rule
fn R_CURLY_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_CURLY, starting from 0.
/// Returns `None` if number of children corresponding to token R_CURLY is less or equal than `i`.
fn R_CURLY(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_CURLY, i)
}
/// Retrieves all `TerminalNode`s corresponding to token L_ANGLE in current rule
fn L_ANGLE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_ANGLE, starting from 0.
/// Returns `None` if number of children corresponding to token L_ANGLE is less or equal than `i`.
fn L_ANGLE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_ANGLE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_ANGLE in current rule
fn R_ANGLE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_ANGLE, starting from 0.
/// Returns `None` if number of children corresponding to token R_ANGLE is less or equal than `i`.
fn R_ANGLE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_ANGLE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NL in current rule
fn NL_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NL, starting from 0.
/// Returns `None` if number of children corresponding to token NL is less or equal than `i`.
fn NL(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, i)
}

}

impl<'input> Code_valueContextAttrs<'input> for Code_valueContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn code_value(&mut self,)
	-> Result<Rc<Code_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Code_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_code_value);
        let mut _localctx: Rc<Code_valueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(126);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << UUID_NAME) | (1usize << LOCATION_NAME) | (1usize << GEO_NAME) | (1usize << TAGS_NAME) | (1usize << UUID_VALUE) | (1usize << DATE) | (1usize << TS) | (1usize << TS_TZ) | (1usize << INT) | (1usize << NUMBER) | (1usize << ID) | (1usize << SUBID) | (1usize << HASH) | (1usize << AT) | (1usize << EQUAL) | (1usize << SPACE) | (1usize << TAB) | (1usize << COMMA) | (1usize << SEMICOLON) | (1usize << COLON) | (1usize << ANYCHAR))) != 0) {
				{
				{
				recog.base.set_state(123);
				_la = recog.base.input.la(1);
				if { _la <= 0 || ((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << QUOTE) | (1usize << L_BRACE) | (1usize << R_BRACE) | (1usize << L_CURLY) | (1usize << R_CURLY) | (1usize << L_SQUARE) | (1usize << R_SQUARE) | (1usize << L_ANGLE) | (1usize << R_ANGLE) | (1usize << NL))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(128);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- description ----------------
pub type DescriptionContextAll<'input> = DescriptionContext<'input>;


pub type DescriptionContext<'input> = BaseParserRuleContext<'input,DescriptionContextExt<'input>>;

#[derive(Clone)]
pub struct DescriptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for DescriptionContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for DescriptionContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_description(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_description(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DescriptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_description }
	//fn type_rule_index() -> usize where Self: Sized { RULE_description }
}
antlr_rust::tid!{DescriptionContextExt<'a>}

impl<'input> DescriptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DescriptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DescriptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DescriptionContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<DescriptionContextExt<'input>>{

fn sp(&self) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QUOTE
/// Returns `None` if there is no child corresponding to token QUOTE
fn QUOTE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(QUOTE, 0)
}
fn text(&self) -> Option<Rc<TextContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DescriptionContextAttrs<'input> for DescriptionContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn description(&mut self,)
	-> Result<Rc<DescriptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DescriptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_description);
        let mut _localctx: Rc<DescriptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule sp*/
			recog.base.set_state(129);
			recog.sp()?;

			recog.base.set_state(130);
			recog.base.match_token(QUOTE,&mut recog.err_handler)?;

			/*InvokeRule text*/
			recog.base.set_state(131);
			recog.text()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- text ----------------
pub type TextContextAll<'input> = TextContext<'input>;


pub type TextContext<'input> = BaseParserRuleContext<'input,TextContextExt<'input>>;

#[derive(Clone)]
pub struct TextContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for TextContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for TextContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_text(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_text(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_text }
	//fn type_rule_index() -> usize where Self: Sized { RULE_text }
}
antlr_rust::tid!{TextContextExt<'a>}

impl<'input> TextContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TextContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TextContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TextContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<TextContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NL in current rule
fn NL_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NL, starting from 0.
/// Returns `None` if number of children corresponding to token NL is less or equal than `i`.
fn NL(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, i)
}

}

impl<'input> TextContextAttrs<'input> for TextContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn text(&mut self,)
	-> Result<Rc<TextContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TextContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_text);
        let mut _localctx: Rc<TextContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << UUID_NAME) | (1usize << LOCATION_NAME) | (1usize << GEO_NAME) | (1usize << TAGS_NAME) | (1usize << UUID_VALUE) | (1usize << DATE) | (1usize << TS) | (1usize << TS_TZ) | (1usize << INT) | (1usize << NUMBER) | (1usize << ID) | (1usize << SUBID) | (1usize << QUOTE) | (1usize << L_BRACE) | (1usize << R_BRACE) | (1usize << L_CURLY) | (1usize << R_CURLY) | (1usize << L_SQUARE) | (1usize << R_SQUARE) | (1usize << L_ANGLE) | (1usize << R_ANGLE) | (1usize << HASH) | (1usize << AT) | (1usize << EQUAL) | (1usize << SPACE) | (1usize << TAB) | (1usize << COMMA) | (1usize << SEMICOLON) | (1usize << COLON) | (1usize << ANYCHAR))) != 0) {
				{
				{
				recog.base.set_state(133);
				_la = recog.base.input.la(1);
				if { _la <= 0 || (_la==NL) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(138);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn_meta ----------------
pub type Txn_metaContextAll<'input> = Txn_metaContext<'input>;


pub type Txn_metaContext<'input> = BaseParserRuleContext<'input,Txn_metaContextExt<'input>>;

#[derive(Clone)]
pub struct Txn_metaContextExt<'input>{
	pub u: i32,
	pub l: i32,
	pub t: i32,
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Txn_metaContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Txn_metaContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn_meta(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn_meta(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Txn_metaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn_meta }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn_meta }
}
antlr_rust::tid!{Txn_metaContextExt<'a>}

impl<'input> Txn_metaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize, u: i32, l: i32, t: i32) -> Rc<Txn_metaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txn_metaContextExt{
				u,l,t,
				ph:PhantomData
			}),
		)
	}
}

pub trait Txn_metaContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Txn_metaContextExt<'input>>{

fn get_u<'a>(&'a self) -> &'a i32 where 'input: 'a { &self.borrow().u }  

fn get_l<'a>(&'a self) -> &'a i32 where 'input: 'a { &self.borrow().l }  

fn get_t<'a>(&'a self) -> &'a i32 where 'input: 'a { &self.borrow().t }  
fn set_u(&mut self,attr: i32) { self.borrow_mut().u = attr; }  

fn set_l(&mut self,attr: i32) { self.borrow_mut().l = attr; }  

fn set_t(&mut self,attr: i32) { self.borrow_mut().t = attr; }  
fn txn_meta_uuid_all(&self) ->  Vec<Rc<Txn_meta_uuidContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txn_meta_uuid(&self, i: usize) -> Option<Rc<Txn_meta_uuidContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NL in current rule
fn NL_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NL, starting from 0.
/// Returns `None` if number of children corresponding to token NL is less or equal than `i`.
fn NL(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, i)
}
fn txn_meta_location_all(&self) ->  Vec<Rc<Txn_meta_locationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txn_meta_location(&self, i: usize) -> Option<Rc<Txn_meta_locationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn txn_meta_tags_all(&self) ->  Vec<Rc<Txn_meta_tagsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txn_meta_tags(&self, i: usize) -> Option<Rc<Txn_meta_tagsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Txn_metaContextAttrs<'input> for Txn_metaContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn_meta(&mut self,u: i32,l: i32,t: i32)
	-> Result<Rc<Txn_metaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txn_metaContextExt::new(_parentctx.clone(), recog.base.get_state(), u, l, t);
        recog.base.enter_rule(_localctx.clone(), 14, RULE_txn_meta);
        let mut _localctx: Rc<Txn_metaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(154); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					recog.base.set_state(154);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(139);
							if !({*_localctx.get_u() < 1}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("$u < 1".to_owned()), None))?;
							}
							/*InvokeRule txn_meta_uuid*/
							recog.base.set_state(140);
							recog.txn_meta_uuid()?;

							recog.base.set_state(141);
							recog.base.match_token(NL,&mut recog.err_handler)?;


							 let tmp = *_localctx.get_u(); let tmp = { (tmp+1)}.to_owned();
							 cast_mut::<_,Txn_metaContext >(&mut _localctx).set_u(tmp);
							  
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(144);
							if !({*_localctx.get_l() < 1}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("$l < 1".to_owned()), None))?;
							}
							/*InvokeRule txn_meta_location*/
							recog.base.set_state(145);
							recog.txn_meta_location()?;

							recog.base.set_state(146);
							recog.base.match_token(NL,&mut recog.err_handler)?;


							 let tmp = *_localctx.get_l(); let tmp = { (tmp+1)}.to_owned();
							 cast_mut::<_,Txn_metaContext >(&mut _localctx).set_l(tmp);
							  
							}
						}
					,
						3 =>{
							{
							recog.base.set_state(149);
							if !({*_localctx.get_t() < 1}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("$t < 1".to_owned()), None))?;
							}
							/*InvokeRule txn_meta_tags*/
							recog.base.set_state(150);
							recog.txn_meta_tags()?;

							recog.base.set_state(151);
							recog.base.match_token(NL,&mut recog.err_handler)?;


							 let tmp = *_localctx.get_t(); let tmp = { (tmp+1)}.to_owned();
							 cast_mut::<_,Txn_metaContext >(&mut _localctx).set_t(tmp);
							  
							}
						}

						_ => {}
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(156); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn_meta_uuid ----------------
pub type Txn_meta_uuidContextAll<'input> = Txn_meta_uuidContext<'input>;


pub type Txn_meta_uuidContext<'input> = BaseParserRuleContext<'input,Txn_meta_uuidContextExt<'input>>;

#[derive(Clone)]
pub struct Txn_meta_uuidContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Txn_meta_uuidContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Txn_meta_uuidContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn_meta_uuid(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn_meta_uuid(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Txn_meta_uuidContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn_meta_uuid }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn_meta_uuid }
}
antlr_rust::tid!{Txn_meta_uuidContextExt<'a>}

impl<'input> Txn_meta_uuidContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Txn_meta_uuidContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txn_meta_uuidContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Txn_meta_uuidContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Txn_meta_uuidContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token HASH
/// Returns `None` if there is no child corresponding to token HASH
fn HASH(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(HASH, 0)
}
fn sp_all(&self) ->  Vec<Rc<SpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sp(&self, i: usize) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token UUID_NAME
/// Returns `None` if there is no child corresponding to token UUID_NAME
fn UUID_NAME(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(UUID_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token UUID_VALUE
/// Returns `None` if there is no child corresponding to token UUID_VALUE
fn UUID_VALUE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(UUID_VALUE, 0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txn_meta_uuidContextAttrs<'input> for Txn_meta_uuidContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn_meta_uuid(&mut self,)
	-> Result<Rc<Txn_meta_uuidContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txn_meta_uuidContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_txn_meta_uuid);
        let mut _localctx: Rc<Txn_meta_uuidContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(158);
			recog.indent()?;

			recog.base.set_state(159);
			recog.base.match_token(HASH,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(160);
			recog.sp()?;

			recog.base.set_state(161);
			recog.base.match_token(UUID_NAME,&mut recog.err_handler)?;

			recog.base.set_state(162);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(163);
			recog.sp()?;

			recog.base.set_state(164);
			recog.base.match_token(UUID_VALUE,&mut recog.err_handler)?;

			/*InvokeRule opt_sp*/
			recog.base.set_state(165);
			recog.opt_sp()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn_meta_location ----------------
pub type Txn_meta_locationContextAll<'input> = Txn_meta_locationContext<'input>;


pub type Txn_meta_locationContext<'input> = BaseParserRuleContext<'input,Txn_meta_locationContextExt<'input>>;

#[derive(Clone)]
pub struct Txn_meta_locationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Txn_meta_locationContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Txn_meta_locationContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn_meta_location(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn_meta_location(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Txn_meta_locationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn_meta_location }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn_meta_location }
}
antlr_rust::tid!{Txn_meta_locationContextExt<'a>}

impl<'input> Txn_meta_locationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Txn_meta_locationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txn_meta_locationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Txn_meta_locationContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Txn_meta_locationContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token HASH
/// Returns `None` if there is no child corresponding to token HASH
fn HASH(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(HASH, 0)
}
fn sp_all(&self) ->  Vec<Rc<SpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sp(&self, i: usize) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LOCATION_NAME
/// Returns `None` if there is no child corresponding to token LOCATION_NAME
fn LOCATION_NAME(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(LOCATION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn geo_uri(&self) -> Option<Rc<Geo_uriContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txn_meta_locationContextAttrs<'input> for Txn_meta_locationContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn_meta_location(&mut self,)
	-> Result<Rc<Txn_meta_locationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txn_meta_locationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_txn_meta_location);
        let mut _localctx: Rc<Txn_meta_locationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(167);
			recog.indent()?;

			recog.base.set_state(168);
			recog.base.match_token(HASH,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(169);
			recog.sp()?;

			recog.base.set_state(170);
			recog.base.match_token(LOCATION_NAME,&mut recog.err_handler)?;

			recog.base.set_state(171);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(172);
			recog.sp()?;

			/*InvokeRule geo_uri*/
			recog.base.set_state(173);
			recog.geo_uri()?;

			/*InvokeRule opt_sp*/
			recog.base.set_state(174);
			recog.opt_sp()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn_meta_tags ----------------
pub type Txn_meta_tagsContextAll<'input> = Txn_meta_tagsContext<'input>;


pub type Txn_meta_tagsContext<'input> = BaseParserRuleContext<'input,Txn_meta_tagsContextExt<'input>>;

#[derive(Clone)]
pub struct Txn_meta_tagsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Txn_meta_tagsContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Txn_meta_tagsContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn_meta_tags(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn_meta_tags(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Txn_meta_tagsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn_meta_tags }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn_meta_tags }
}
antlr_rust::tid!{Txn_meta_tagsContextExt<'a>}

impl<'input> Txn_meta_tagsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Txn_meta_tagsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txn_meta_tagsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Txn_meta_tagsContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Txn_meta_tagsContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token HASH
/// Returns `None` if there is no child corresponding to token HASH
fn HASH(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(HASH, 0)
}
fn sp_all(&self) ->  Vec<Rc<SpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sp(&self, i: usize) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token TAGS_NAME
/// Returns `None` if there is no child corresponding to token TAGS_NAME
fn TAGS_NAME(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TAGS_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn tags(&self) -> Option<Rc<TagsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txn_meta_tagsContextAttrs<'input> for Txn_meta_tagsContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn_meta_tags(&mut self,)
	-> Result<Rc<Txn_meta_tagsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txn_meta_tagsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_txn_meta_tags);
        let mut _localctx: Rc<Txn_meta_tagsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(176);
			recog.indent()?;

			recog.base.set_state(177);
			recog.base.match_token(HASH,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(178);
			recog.sp()?;

			recog.base.set_state(179);
			recog.base.match_token(TAGS_NAME,&mut recog.err_handler)?;

			recog.base.set_state(180);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule sp*/
			recog.base.set_state(181);
			recog.sp()?;

			/*InvokeRule tags*/
			recog.base.set_state(182);
			recog.tags_rec(0)?;

			/*InvokeRule opt_sp*/
			recog.base.set_state(183);
			recog.opt_sp()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- geo_uri ----------------
pub type Geo_uriContextAll<'input> = Geo_uriContext<'input>;


pub type Geo_uriContext<'input> = BaseParserRuleContext<'input,Geo_uriContextExt<'input>>;

#[derive(Clone)]
pub struct Geo_uriContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Geo_uriContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Geo_uriContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_geo_uri(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_geo_uri(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Geo_uriContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_geo_uri }
	//fn type_rule_index() -> usize where Self: Sized { RULE_geo_uri }
}
antlr_rust::tid!{Geo_uriContextExt<'a>}

impl<'input> Geo_uriContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Geo_uriContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Geo_uriContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Geo_uriContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Geo_uriContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GEO_NAME
/// Returns `None` if there is no child corresponding to token GEO_NAME
fn GEO_NAME(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(GEO_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn lat(&self) -> Option<Rc<LatContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn lon(&self) -> Option<Rc<LonContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn alt(&self) -> Option<Rc<AltContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Geo_uriContextAttrs<'input> for Geo_uriContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn geo_uri(&mut self,)
	-> Result<Rc<Geo_uriContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Geo_uriContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_geo_uri);
        let mut _localctx: Rc<Geo_uriContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(185);
			recog.base.match_token(GEO_NAME,&mut recog.err_handler)?;

			recog.base.set_state(186);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule lat*/
			recog.base.set_state(187);
			recog.lat()?;

			recog.base.set_state(188);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule lon*/
			recog.base.set_state(189);
			recog.lon()?;

			recog.base.set_state(192);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(190);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule alt*/
				recog.base.set_state(191);
				recog.alt()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lat ----------------
pub type LatContextAll<'input> = LatContext<'input>;


pub type LatContext<'input> = BaseParserRuleContext<'input,LatContextExt<'input>>;

#[derive(Clone)]
pub struct LatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for LatContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for LatContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lat(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_lat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lat }
}
antlr_rust::tid!{LatContextExt<'a>}

impl<'input> LatContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LatContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LatContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<LatContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> LatContextAttrs<'input> for LatContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lat(&mut self,)
	-> Result<Rc<LatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_lat);
        let mut _localctx: Rc<LatContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(194);
			_la = recog.base.input.la(1);
			if { !(_la==INT || _la==NUMBER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lon ----------------
pub type LonContextAll<'input> = LonContext<'input>;


pub type LonContext<'input> = BaseParserRuleContext<'input,LonContextExt<'input>>;

#[derive(Clone)]
pub struct LonContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for LonContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for LonContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lon(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_lon(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lon }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lon }
}
antlr_rust::tid!{LonContextExt<'a>}

impl<'input> LonContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LonContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LonContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LonContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<LonContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> LonContextAttrs<'input> for LonContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lon(&mut self,)
	-> Result<Rc<LonContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_lon);
        let mut _localctx: Rc<LonContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(196);
			_la = recog.base.input.la(1);
			if { !(_la==INT || _la==NUMBER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- alt ----------------
pub type AltContextAll<'input> = AltContext<'input>;


pub type AltContext<'input> = BaseParserRuleContext<'input,AltContextExt<'input>>;

#[derive(Clone)]
pub struct AltContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for AltContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for AltContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_alt(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_alt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AltContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_alt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_alt }
}
antlr_rust::tid!{AltContextExt<'a>}

impl<'input> AltContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AltContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AltContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AltContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<AltContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> AltContextAttrs<'input> for AltContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn alt(&mut self,)
	-> Result<Rc<AltContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AltContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_alt);
        let mut _localctx: Rc<AltContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(198);
			_la = recog.base.input.la(1);
			if { !(_la==INT || _la==NUMBER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tags ----------------
pub type TagsContextAll<'input> = TagsContext<'input>;


pub type TagsContext<'input> = BaseParserRuleContext<'input,TagsContextExt<'input>>;

#[derive(Clone)]
pub struct TagsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for TagsContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for TagsContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tags(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_tags(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TagsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tags }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tags }
}
antlr_rust::tid!{TagsContextExt<'a>}

impl<'input> TagsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TagsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TagsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TagsContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<TagsContextExt<'input>>{

fn tag(&self) -> Option<Rc<TagContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tags(&self) -> Option<Rc<TagsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp_all(&self) ->  Vec<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn opt_sp(&self, i: usize) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> TagsContextAttrs<'input> for TagsContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  tags(&mut self,)
	-> Result<Rc<TagsContextAll<'input>>,ANTLRError> {
		self.tags_rec(0)
	}

	fn tags_rec(&mut self, _p: isize)
	-> Result<Rc<TagsContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = TagsContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 30, RULE_tags, _p);
	    let mut _localctx: Rc<TagsContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 30;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tag*/
			recog.base.set_state(201);
			recog.tag()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(211);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = TagsContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tags);
					_localctx = tmp;
					recog.base.set_state(203);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule opt_sp*/
					recog.base.set_state(204);
					recog.opt_sp()?;

					recog.base.set_state(205);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule opt_sp*/
					recog.base.set_state(206);
					recog.opt_sp()?;

					/*InvokeRule tag*/
					recog.base.set_state(207);
					recog.tag()?;

					}
					} 
				}
				recog.base.set_state(213);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- tag ----------------
pub type TagContextAll<'input> = TagContext<'input>;


pub type TagContext<'input> = BaseParserRuleContext<'input,TagContextExt<'input>>;

#[derive(Clone)]
pub struct TagContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for TagContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for TagContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tag(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_tag(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TagContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tag }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tag }
}
antlr_rust::tid!{TagContextExt<'a>}

impl<'input> TagContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TagContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TagContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TagContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<TagContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token ID in current rule
fn ID_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
/// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(ID, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SUBID in current rule
fn SUBID_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SUBID, starting from 0.
/// Returns `None` if number of children corresponding to token SUBID is less or equal than `i`.
fn SUBID(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SUBID, i)
}
/// Retrieves all `TerminalNode`s corresponding to token INT in current rule
fn INT_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INT, starting from 0.
/// Returns `None` if number of children corresponding to token INT is less or equal than `i`.
fn INT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, i)
}

}

impl<'input> TagContextAttrs<'input> for TagContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tag(&mut self,)
	-> Result<Rc<TagContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_tag);
        let mut _localctx: Rc<TagContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(214);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(219);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(215);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(216);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << INT) | (1usize << ID) | (1usize << SUBID))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
					} 
				}
				recog.base.set_state(221);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- txn_comment ----------------
pub type Txn_commentContextAll<'input> = Txn_commentContext<'input>;


pub type Txn_commentContext<'input> = BaseParserRuleContext<'input,Txn_commentContextExt<'input>>;

#[derive(Clone)]
pub struct Txn_commentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Txn_commentContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Txn_commentContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_txn_comment(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_txn_comment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Txn_commentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txn_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txn_comment }
}
antlr_rust::tid!{Txn_commentContextExt<'a>}

impl<'input> Txn_commentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Txn_commentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txn_commentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Txn_commentContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Txn_commentContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NL
/// Returns `None` if there is no child corresponding to token NL
fn NL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, 0)
}

}

impl<'input> Txn_commentContextAttrs<'input> for Txn_commentContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn txn_comment(&mut self,)
	-> Result<Rc<Txn_commentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txn_commentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_txn_comment);
        let mut _localctx: Rc<Txn_commentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(222);
			recog.indent()?;

			/*InvokeRule comment*/
			recog.base.set_state(223);
			recog.comment()?;

			recog.base.set_state(224);
			recog.base.match_token(NL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- indent ----------------
pub type IndentContextAll<'input> = IndentContext<'input>;


pub type IndentContext<'input> = BaseParserRuleContext<'input,IndentContextExt<'input>>;

#[derive(Clone)]
pub struct IndentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for IndentContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for IndentContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_indent(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_indent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IndentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_indent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_indent }
}
antlr_rust::tid!{IndentContextExt<'a>}

impl<'input> IndentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IndentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IndentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IndentContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<IndentContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SPACE in current rule
fn SPACE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SPACE, starting from 0.
/// Returns `None` if number of children corresponding to token SPACE is less or equal than `i`.
fn SPACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SPACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token TAB in current rule
fn TAB_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TAB, starting from 0.
/// Returns `None` if number of children corresponding to token TAB is less or equal than `i`.
fn TAB(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TAB, i)
}

}

impl<'input> IndentContextAttrs<'input> for IndentContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn indent(&mut self,)
	-> Result<Rc<IndentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IndentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_indent);
        let mut _localctx: Rc<IndentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(227); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(226);
				_la = recog.base.input.la(1);
				if { !(_la==SPACE || _la==TAB) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(229); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==SPACE || _la==TAB) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comment ----------------
pub type CommentContextAll<'input> = CommentContext<'input>;


pub type CommentContext<'input> = BaseParserRuleContext<'input,CommentContextExt<'input>>;

#[derive(Clone)]
pub struct CommentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for CommentContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for CommentContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comment(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_comment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CommentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comment }
}
antlr_rust::tid!{CommentContextExt<'a>}

impl<'input> CommentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommentContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<CommentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token SPACE
/// Returns `None` if there is no child corresponding to token SPACE
fn SPACE(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SPACE, 0)
}
fn text(&self) -> Option<Rc<TextContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CommentContextAttrs<'input> for CommentContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comment(&mut self,)
	-> Result<Rc<CommentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_comment);
        let mut _localctx: Rc<CommentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(231);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			recog.base.set_state(232);
			recog.base.match_token(SPACE,&mut recog.err_handler)?;

			/*InvokeRule text*/
			recog.base.set_state(233);
			recog.text()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postings ----------------
pub type PostingsContextAll<'input> = PostingsContext<'input>;


pub type PostingsContext<'input> = BaseParserRuleContext<'input,PostingsContextExt<'input>>;

#[derive(Clone)]
pub struct PostingsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for PostingsContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for PostingsContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postings(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_postings(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PostingsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postings }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postings }
}
antlr_rust::tid!{PostingsContextExt<'a>}

impl<'input> PostingsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostingsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostingsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostingsContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<PostingsContextExt<'input>>{

fn posting_all(&self) ->  Vec<Rc<PostingContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn posting(&self, i: usize) -> Option<Rc<PostingContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn last_posting(&self) -> Option<Rc<Last_postingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PostingsContextAttrs<'input> for PostingsContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postings(&mut self,)
	-> Result<Rc<PostingsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostingsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_postings);
        let mut _localctx: Rc<PostingsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(236); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule posting*/
					recog.base.set_state(235);
					recog.posting()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(238); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			recog.base.set_state(242);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule posting*/
					recog.base.set_state(240);
					recog.posting()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule last_posting*/
					recog.base.set_state(241);
					recog.last_posting()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- posting ----------------
pub type PostingContextAll<'input> = PostingContext<'input>;


pub type PostingContext<'input> = BaseParserRuleContext<'input,PostingContextExt<'input>>;

#[derive(Clone)]
pub struct PostingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for PostingContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for PostingContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_posting(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_posting(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PostingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_posting }
	//fn type_rule_index() -> usize where Self: Sized { RULE_posting }
}
antlr_rust::tid!{PostingContextExt<'a>}

impl<'input> PostingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostingContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<PostingContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn account(&self) -> Option<Rc<AccountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sp(&self) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn amount(&self) -> Option<Rc<AmountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NL
/// Returns `None` if there is no child corresponding to token NL
fn NL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, 0)
}
fn opt_comment(&self) -> Option<Rc<Opt_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_unit(&self) -> Option<Rc<Opt_unitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PostingContextAttrs<'input> for PostingContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn posting(&mut self,)
	-> Result<Rc<PostingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_posting);
        let mut _localctx: Rc<PostingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(244);
			recog.indent()?;

			/*InvokeRule account*/
			recog.base.set_state(245);
			recog.account()?;

			/*InvokeRule sp*/
			recog.base.set_state(246);
			recog.sp()?;

			/*InvokeRule amount*/
			recog.base.set_state(247);
			recog.amount()?;

			recog.base.set_state(249);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule opt_unit*/
					recog.base.set_state(248);
					recog.opt_unit()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(253);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule opt_comment*/
					recog.base.set_state(251);
					recog.opt_comment()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule opt_sp*/
					recog.base.set_state(252);
					recog.opt_sp()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(255);
			recog.base.match_token(NL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- last_posting ----------------
pub type Last_postingContextAll<'input> = Last_postingContext<'input>;


pub type Last_postingContext<'input> = BaseParserRuleContext<'input,Last_postingContextExt<'input>>;

#[derive(Clone)]
pub struct Last_postingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Last_postingContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Last_postingContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_last_posting(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_last_posting(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Last_postingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_last_posting }
	//fn type_rule_index() -> usize where Self: Sized { RULE_last_posting }
}
antlr_rust::tid!{Last_postingContextExt<'a>}

impl<'input> Last_postingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Last_postingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Last_postingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Last_postingContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Last_postingContextExt<'input>>{

fn indent(&self) -> Option<Rc<IndentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn account(&self) -> Option<Rc<AccountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NL
/// Returns `None` if there is no child corresponding to token NL
fn NL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, 0)
}
fn opt_comment(&self) -> Option<Rc<Opt_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Last_postingContextAttrs<'input> for Last_postingContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn last_posting(&mut self,)
	-> Result<Rc<Last_postingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Last_postingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_last_posting);
        let mut _localctx: Rc<Last_postingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule indent*/
			recog.base.set_state(257);
			recog.indent()?;

			/*InvokeRule account*/
			recog.base.set_state(258);
			recog.account()?;

			recog.base.set_state(261);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule opt_comment*/
					recog.base.set_state(259);
					recog.opt_comment()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule opt_sp*/
					recog.base.set_state(260);
					recog.opt_sp()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(263);
			recog.base.match_token(NL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- opt_unit ----------------
pub type Opt_unitContextAll<'input> = Opt_unitContext<'input>;


pub type Opt_unitContext<'input> = BaseParserRuleContext<'input,Opt_unitContextExt<'input>>;

#[derive(Clone)]
pub struct Opt_unitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Opt_unitContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Opt_unitContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_opt_unit(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_opt_unit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Opt_unitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_opt_unit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_opt_unit }
}
antlr_rust::tid!{Opt_unitContextExt<'a>}

impl<'input> Opt_unitContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Opt_unitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Opt_unitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Opt_unitContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Opt_unitContextExt<'input>>{

fn sp(&self) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unit(&self) -> Option<Rc<UnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn opt_position(&self) -> Option<Rc<Opt_positionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Opt_unitContextAttrs<'input> for Opt_unitContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn opt_unit(&mut self,)
	-> Result<Rc<Opt_unitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Opt_unitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_opt_unit);
        let mut _localctx: Rc<Opt_unitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule sp*/
			recog.base.set_state(265);
			recog.sp()?;

			/*InvokeRule unit*/
			recog.base.set_state(266);
			recog.unit()?;

			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule opt_position*/
					recog.base.set_state(267);
					recog.opt_position()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- opt_comment ----------------
pub type Opt_commentContextAll<'input> = Opt_commentContext<'input>;


pub type Opt_commentContext<'input> = BaseParserRuleContext<'input,Opt_commentContextExt<'input>>;

#[derive(Clone)]
pub struct Opt_commentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Opt_commentContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Opt_commentContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_opt_comment(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_opt_comment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Opt_commentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_opt_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_opt_comment }
}
antlr_rust::tid!{Opt_commentContextExt<'a>}

impl<'input> Opt_commentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Opt_commentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Opt_commentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Opt_commentContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Opt_commentContextExt<'input>>{

fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Opt_commentContextAttrs<'input> for Opt_commentContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn opt_comment(&mut self,)
	-> Result<Rc<Opt_commentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Opt_commentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_opt_comment);
        let mut _localctx: Rc<Opt_commentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule opt_sp*/
			recog.base.set_state(270);
			recog.opt_sp()?;

			/*InvokeRule comment*/
			recog.base.set_state(271);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- opt_position ----------------
pub type Opt_positionContextAll<'input> = Opt_positionContext<'input>;


pub type Opt_positionContext<'input> = BaseParserRuleContext<'input,Opt_positionContextExt<'input>>;

#[derive(Clone)]
pub struct Opt_positionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Opt_positionContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Opt_positionContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_opt_position(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_opt_position(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Opt_positionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_opt_position }
	//fn type_rule_index() -> usize where Self: Sized { RULE_opt_position }
}
antlr_rust::tid!{Opt_positionContextExt<'a>}

impl<'input> Opt_positionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Opt_positionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Opt_positionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Opt_positionContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Opt_positionContextExt<'input>>{

fn opt_opening_pos(&self) -> Option<Rc<Opt_opening_posContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn closing_pos(&self) -> Option<Rc<Closing_posContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Opt_positionContextAttrs<'input> for Opt_positionContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn opt_position(&mut self,)
	-> Result<Rc<Opt_positionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Opt_positionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_opt_position);
        let mut _localctx: Rc<Opt_positionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(278);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule opt_opening_pos*/
					recog.base.set_state(273);
					recog.opt_opening_pos()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule opt_opening_pos*/
					recog.base.set_state(274);
					recog.opt_opening_pos()?;

					/*InvokeRule closing_pos*/
					recog.base.set_state(275);
					recog.closing_pos()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule closing_pos*/
					recog.base.set_state(277);
					recog.closing_pos()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- opt_opening_pos ----------------
pub type Opt_opening_posContextAll<'input> = Opt_opening_posContext<'input>;


pub type Opt_opening_posContext<'input> = BaseParserRuleContext<'input,Opt_opening_posContextExt<'input>>;

#[derive(Clone)]
pub struct Opt_opening_posContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Opt_opening_posContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Opt_opening_posContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_opt_opening_pos(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_opt_opening_pos(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Opt_opening_posContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_opt_opening_pos }
	//fn type_rule_index() -> usize where Self: Sized { RULE_opt_opening_pos }
}
antlr_rust::tid!{Opt_opening_posContextExt<'a>}

impl<'input> Opt_opening_posContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Opt_opening_posContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Opt_opening_posContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Opt_opening_posContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Opt_opening_posContextExt<'input>>{

fn sp_all(&self) ->  Vec<Rc<SpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sp(&self, i: usize) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token L_CURLY
/// Returns `None` if there is no child corresponding to token L_CURLY
fn L_CURLY(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(L_CURLY, 0)
}
fn opt_sp_all(&self) ->  Vec<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn opt_sp(&self, i: usize) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn amount(&self) -> Option<Rc<AmountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unit(&self) -> Option<Rc<UnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_CURLY
/// Returns `None` if there is no child corresponding to token R_CURLY
fn R_CURLY(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(R_CURLY, 0)
}

}

impl<'input> Opt_opening_posContextAttrs<'input> for Opt_opening_posContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn opt_opening_pos(&mut self,)
	-> Result<Rc<Opt_opening_posContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Opt_opening_posContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_opt_opening_pos);
        let mut _localctx: Rc<Opt_opening_posContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule sp*/
			recog.base.set_state(280);
			recog.sp()?;

			recog.base.set_state(281);
			recog.base.match_token(L_CURLY,&mut recog.err_handler)?;

			/*InvokeRule opt_sp*/
			recog.base.set_state(282);
			recog.opt_sp()?;

			/*InvokeRule amount*/
			recog.base.set_state(283);
			recog.amount()?;

			/*InvokeRule sp*/
			recog.base.set_state(284);
			recog.sp()?;

			/*InvokeRule unit*/
			recog.base.set_state(285);
			recog.unit()?;

			/*InvokeRule opt_sp*/
			recog.base.set_state(286);
			recog.opt_sp()?;

			recog.base.set_state(287);
			recog.base.match_token(R_CURLY,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- closing_pos ----------------
pub type Closing_posContextAll<'input> = Closing_posContext<'input>;


pub type Closing_posContext<'input> = BaseParserRuleContext<'input,Closing_posContextExt<'input>>;

#[derive(Clone)]
pub struct Closing_posContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Closing_posContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Closing_posContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_closing_pos(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_closing_pos(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Closing_posContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_closing_pos }
	//fn type_rule_index() -> usize where Self: Sized { RULE_closing_pos }
}
antlr_rust::tid!{Closing_posContextExt<'a>}

impl<'input> Closing_posContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Closing_posContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Closing_posContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Closing_posContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Closing_posContextExt<'input>>{

fn sp_all(&self) ->  Vec<Rc<SpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn sp(&self, i: usize) -> Option<Rc<SpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn amount(&self) -> Option<Rc<AmountContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unit(&self) -> Option<Rc<UnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}

}

impl<'input> Closing_posContextAttrs<'input> for Closing_posContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn closing_pos(&mut self,)
	-> Result<Rc<Closing_posContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Closing_posContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_closing_pos);
        let mut _localctx: Rc<Closing_posContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule sp*/
			recog.base.set_state(289);
			recog.sp()?;

			recog.base.set_state(290);
			_la = recog.base.input.la(1);
			if { !(_la==AT || _la==EQUAL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule sp*/
			recog.base.set_state(291);
			recog.sp()?;

			/*InvokeRule amount*/
			recog.base.set_state(292);
			recog.amount()?;

			/*InvokeRule sp*/
			recog.base.set_state(293);
			recog.sp()?;

			/*InvokeRule unit*/
			recog.base.set_state(294);
			recog.unit()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- account ----------------
pub type AccountContextAll<'input> = AccountContext<'input>;


pub type AccountContext<'input> = BaseParserRuleContext<'input,AccountContextExt<'input>>;

#[derive(Clone)]
pub struct AccountContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for AccountContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for AccountContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_account(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_account(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AccountContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_account }
	//fn type_rule_index() -> usize where Self: Sized { RULE_account }
}
antlr_rust::tid!{AccountContextExt<'a>}

impl<'input> AccountContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AccountContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AccountContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AccountContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<AccountContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token ID in current rule
fn ID_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
/// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(ID, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
/// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(COLON, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SUBID in current rule
fn SUBID_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SUBID, starting from 0.
/// Returns `None` if number of children corresponding to token SUBID is less or equal than `i`.
fn SUBID(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SUBID, i)
}
/// Retrieves all `TerminalNode`s corresponding to token INT in current rule
fn INT_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INT, starting from 0.
/// Returns `None` if number of children corresponding to token INT is less or equal than `i`.
fn INT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, i)
}

}

impl<'input> AccountContextAttrs<'input> for AccountContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn account(&mut self,)
	-> Result<Rc<AccountContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AccountContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_account);
        let mut _localctx: Rc<AccountContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(296);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(301);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COLON {
				{
				{
				recog.base.set_state(297);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				recog.base.set_state(298);
				_la = recog.base.input.la(1);
				if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << INT) | (1usize << ID) | (1usize << SUBID))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(303);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- amount ----------------
pub type AmountContextAll<'input> = AmountContext<'input>;


pub type AmountContext<'input> = BaseParserRuleContext<'input,AmountContextExt<'input>>;

#[derive(Clone)]
pub struct AmountContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for AmountContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for AmountContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_amount(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_amount(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AmountContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_amount }
	//fn type_rule_index() -> usize where Self: Sized { RULE_amount }
}
antlr_rust::tid!{AmountContextExt<'a>}

impl<'input> AmountContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AmountContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AmountContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AmountContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<AmountContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> AmountContextAttrs<'input> for AmountContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn amount(&mut self,)
	-> Result<Rc<AmountContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AmountContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_amount);
        let mut _localctx: Rc<AmountContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(304);
			_la = recog.base.input.la(1);
			if { !(_la==INT || _la==NUMBER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unit ----------------
pub type UnitContextAll<'input> = UnitContext<'input>;


pub type UnitContext<'input> = BaseParserRuleContext<'input,UnitContextExt<'input>>;

#[derive(Clone)]
pub struct UnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for UnitContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for UnitContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unit(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_unit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for UnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unit }
}
antlr_rust::tid!{UnitContextExt<'a>}

impl<'input> UnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnitContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<UnitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> UnitContextAttrs<'input> for UnitContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unit(&mut self,)
	-> Result<Rc<UnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_unit);
        let mut _localctx: Rc<UnitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(306);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sp ----------------
pub type SpContextAll<'input> = SpContext<'input>;


pub type SpContext<'input> = BaseParserRuleContext<'input,SpContextExt<'input>>;

#[derive(Clone)]
pub struct SpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for SpContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for SpContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sp(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_sp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sp }
}
antlr_rust::tid!{SpContextExt<'a>}

impl<'input> SpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SpContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<SpContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SPACE in current rule
fn SPACE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SPACE, starting from 0.
/// Returns `None` if number of children corresponding to token SPACE is less or equal than `i`.
fn SPACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SPACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token TAB in current rule
fn TAB_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TAB, starting from 0.
/// Returns `None` if number of children corresponding to token TAB is less or equal than `i`.
fn TAB(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TAB, i)
}

}

impl<'input> SpContextAttrs<'input> for SpContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sp(&mut self,)
	-> Result<Rc<SpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_sp);
        let mut _localctx: Rc<SpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(309); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(308);
				_la = recog.base.input.la(1);
				if { !(_la==SPACE || _la==TAB) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(311); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==SPACE || _la==TAB) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- opt_sp ----------------
pub type Opt_spContextAll<'input> = Opt_spContext<'input>;


pub type Opt_spContext<'input> = BaseParserRuleContext<'input,Opt_spContextExt<'input>>;

#[derive(Clone)]
pub struct Opt_spContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for Opt_spContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for Opt_spContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_opt_sp(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_opt_sp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Opt_spContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_opt_sp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_opt_sp }
}
antlr_rust::tid!{Opt_spContextExt<'a>}

impl<'input> Opt_spContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Opt_spContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Opt_spContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Opt_spContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<Opt_spContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SPACE in current rule
fn SPACE_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SPACE, starting from 0.
/// Returns `None` if number of children corresponding to token SPACE is less or equal than `i`.
fn SPACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(SPACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token TAB in current rule
fn TAB_all(&self) -> Vec<Rc<TerminalNode<'input,TxnParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TAB, starting from 0.
/// Returns `None` if number of children corresponding to token TAB is less or equal than `i`.
fn TAB(&self, i: usize) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(TAB, i)
}

}

impl<'input> Opt_spContextAttrs<'input> for Opt_spContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn opt_sp(&mut self,)
	-> Result<Rc<Opt_spContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Opt_spContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_opt_sp);
        let mut _localctx: Rc<Opt_spContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(316);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SPACE || _la==TAB {
				{
				{
				recog.base.set_state(313);
				_la = recog.base.input.la(1);
				if { !(_la==SPACE || _la==TAB) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
				}
				recog.base.set_state(318);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- blankline ----------------
pub type BlanklineContextAll<'input> = BlanklineContext<'input>;


pub type BlanklineContext<'input> = BaseParserRuleContext<'input,BlanklineContextExt<'input>>;

#[derive(Clone)]
pub struct BlanklineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TxnParserContext<'input> for BlanklineContext<'input>{}

impl<'input,'a> Listenable<dyn TxnParserListener<'input> + 'a> for BlanklineContext<'input>{
		fn enter(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_blankline(self);
		}fn exit(&self,listener: &mut (dyn TxnParserListener<'input> + 'a)) {
			listener.exit_blankline(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BlanklineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TxnParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blankline }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blankline }
}
antlr_rust::tid!{BlanklineContextExt<'a>}

impl<'input> BlanklineContextExt<'input>{
	fn new(parent: Option<Rc<dyn TxnParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlanklineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlanklineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlanklineContextAttrs<'input>: TxnParserContext<'input> + BorrowMut<BlanklineContextExt<'input>>{

fn opt_sp(&self) -> Option<Rc<Opt_spContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NL
/// Returns `None` if there is no child corresponding to token NL
fn NL(&self) -> Option<Rc<TerminalNode<'input,TxnParserContextType>>> where Self:Sized{
	self.get_token(NL, 0)
}

}

impl<'input> BlanklineContextAttrs<'input> for BlanklineContext<'input>{}

impl<'input, I, H> TxnParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blankline(&mut self,)
	-> Result<Rc<BlanklineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlanklineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_blankline);
        let mut _localctx: Rc<BlanklineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule opt_sp*/
			recog.base.set_state(319);
			recog.opt_sp()?;

			recog.base.set_state(320);
			recog.base.match_token(NL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x21\u{145}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x03\x02\x07\x02\x48\
	\x0a\x02\x0c\x02\x0e\x02\x4b\x0b\x02\x03\x02\x03\x02\x06\x02\x4f\x0a\x02\
	\x0d\x02\x0e\x02\x50\x03\x02\x03\x02\x07\x02\x55\x0a\x02\x0c\x02\x0e\x02\
	\x58\x0b\x02\x03\x02\x07\x02\x5b\x0a\x02\x0c\x02\x0e\x02\x5e\x0b\x02\x03\
	\x02\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\x65\x0a\x03\x03\x03\x03\x03\
	\x05\x03\x69\x0a\x03\x03\x03\x03\x03\x05\x03\x6d\x0a\x03\x03\x03\x07\x03\
	\x70\x0a\x03\x0c\x03\x0e\x03\x73\x0b\x03\x03\x03\x03\x03\x03\x04\x03\x04\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x07\x06\x7f\x0a\x06\x0c\
	\x06\x0e\x06\u{82}\x0b\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x07\x08\
	\u{89}\x0a\x08\x0c\x08\x0e\x08\u{8c}\x0b\x08\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x06\x09\u{9d}\x0a\x09\x0d\x09\x0e\x09\u{9e}\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{c3}\x0a\x0d\x03\
	\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x07\x11\u{d4}\x0a\x11\x0c\x11\
	\x0e\x11\u{d7}\x0b\x11\x03\x12\x03\x12\x03\x12\x07\x12\u{dc}\x0a\x12\x0c\
	\x12\x0e\x12\u{df}\x0b\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x06\x14\
	\u{e6}\x0a\x14\x0d\x14\x0e\x14\u{e7}\x03\x15\x03\x15\x03\x15\x03\x15\x03\
	\x16\x06\x16\u{ef}\x0a\x16\x0d\x16\x0e\x16\u{f0}\x03\x16\x03\x16\x05\x16\
	\u{f5}\x0a\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{fc}\x0a\
	\x17\x03\x17\x03\x17\x05\x17\u{100}\x0a\x17\x03\x17\x03\x17\x03\x18\x03\
	\x18\x03\x18\x03\x18\x05\x18\u{108}\x0a\x18\x03\x18\x03\x18\x03\x19\x03\
	\x19\x03\x19\x05\x19\u{10f}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{119}\x0a\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\
	\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{12e}\
	\x0a\x1e\x0c\x1e\x0e\x1e\u{131}\x0b\x1e\x03\x1f\x03\x1f\x03\x20\x03\x20\
	\x03\x21\x06\x21\u{138}\x0a\x21\x0d\x21\x0e\x21\u{139}\x03\x22\x07\x22\u{13d}\
	\x0a\x22\x0c\x22\x0e\x22\u{140}\x0b\x22\x03\x23\x03\x23\x03\x23\x03\x23\
	\x02\x03\x20\x24\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\
	\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\
	\x42\x44\x02\x09\x03\x02\x08\x0a\x04\x02\x0f\x17\x20\x20\x03\x02\x20\x20\
	\x03\x02\x0b\x0c\x04\x02\x0b\x0b\x0d\x0e\x03\x02\x1b\x1c\x03\x02\x19\x1a\
	\x02\u{13e}\x02\x49\x03\x02\x02\x02\x04\x62\x03\x02\x02\x02\x06\x76\x03\
	\x02\x02\x02\x08\x78\x03\x02\x02\x02\x0a\u{80}\x03\x02\x02\x02\x0c\u{83}\
	\x03\x02\x02\x02\x0e\u{8a}\x03\x02\x02\x02\x10\u{9c}\x03\x02\x02\x02\x12\
	\u{a0}\x03\x02\x02\x02\x14\u{a9}\x03\x02\x02\x02\x16\u{b2}\x03\x02\x02\x02\
	\x18\u{bb}\x03\x02\x02\x02\x1a\u{c4}\x03\x02\x02\x02\x1c\u{c6}\x03\x02\x02\
	\x02\x1e\u{c8}\x03\x02\x02\x02\x20\u{ca}\x03\x02\x02\x02\x22\u{d8}\x03\x02\
	\x02\x02\x24\u{e0}\x03\x02\x02\x02\x26\u{e5}\x03\x02\x02\x02\x28\u{e9}\x03\
	\x02\x02\x02\x2a\u{ee}\x03\x02\x02\x02\x2c\u{f6}\x03\x02\x02\x02\x2e\u{103}\
	\x03\x02\x02\x02\x30\u{10b}\x03\x02\x02\x02\x32\u{110}\x03\x02\x02\x02\x34\
	\u{118}\x03\x02\x02\x02\x36\u{11a}\x03\x02\x02\x02\x38\u{123}\x03\x02\x02\
	\x02\x3a\u{12a}\x03\x02\x02\x02\x3c\u{132}\x03\x02\x02\x02\x3e\u{134}\x03\
	\x02\x02\x02\x40\u{137}\x03\x02\x02\x02\x42\u{13e}\x03\x02\x02\x02\x44\u{141}\
	\x03\x02\x02\x02\x46\x48\x05\x44\x23\x02\x47\x46\x03\x02\x02\x02\x48\x4b\
	\x03\x02\x02\x02\x49\x47\x03\x02\x02\x02\x49\x4a\x03\x02\x02\x02\x4a\x4c\
	\x03\x02\x02\x02\x4b\x49\x03\x02\x02\x02\x4c\x56\x05\x04\x03\x02\x4d\x4f\
	\x05\x44\x23\x02\x4e\x4d\x03\x02\x02\x02\x4f\x50\x03\x02\x02\x02\x50\x4e\
	\x03\x02\x02\x02\x50\x51\x03\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x53\
	\x05\x04\x03\x02\x53\x55\x03\x02\x02\x02\x54\x4e\x03\x02\x02\x02\x55\x58\
	\x03\x02\x02\x02\x56\x54\x03\x02\x02\x02\x56\x57\x03\x02\x02\x02\x57\x5c\
	\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\x59\x5b\x05\x44\x23\x02\x5a\x59\
	\x03\x02\x02\x02\x5b\x5e\x03\x02\x02\x02\x5c\x5a\x03\x02\x02\x02\x5c\x5d\
	\x03\x02\x02\x02\x5d\x5f\x03\x02\x02\x02\x5e\x5c\x03\x02\x02\x02\x5f\x60\
	\x05\x42\x22\x02\x60\x61\x07\x02\x02\x03\x61\x03\x03\x02\x02\x02\x62\x64\
	\x05\x06\x04\x02\x63\x65\x05\x08\x05\x02\x64\x63\x03\x02\x02\x02\x64\x65\
	\x03\x02\x02\x02\x65\x68\x03\x02\x02\x02\x66\x69\x05\x0c\x07\x02\x67\x69\
	\x05\x42\x22\x02\x68\x66\x03\x02\x02\x02\x68\x67\x03\x02\x02\x02\x69\x6a\
	\x03\x02\x02\x02\x6a\x6c\x07\x20\x02\x02\x6b\x6d\x05\x10\x09\x02\x6c\x6b\
	\x03\x02\x02\x02\x6c\x6d\x03\x02\x02\x02\x6d\x71\x03\x02\x02\x02\x6e\x70\
	\x05\x24\x13\x02\x6f\x6e\x03\x02\x02\x02\x70\x73\x03\x02\x02\x02\x71\x6f\
	\x03\x02\x02\x02\x71\x72\x03\x02\x02\x02\x72\x74\x03\x02\x02\x02\x73\x71\
	\x03\x02\x02\x02\x74\x75\x05\x2a\x16\x02\x75\x05\x03\x02\x02\x02\x76\x77\
	\x09\x02\x02\x02\x77\x07\x03\x02\x02\x02\x78\x79\x05\x40\x21\x02\x79\x7a\
	\x07\x10\x02\x02\x7a\x7b\x05\x0a\x06\x02\x7b\x7c\x07\x11\x02\x02\x7c\x09\
	\x03\x02\x02\x02\x7d\x7f\x0a\x03\x02\x02\x7e\x7d\x03\x02\x02\x02\x7f\u{82}\
	\x03\x02\x02\x02\u{80}\x7e\x03\x02\x02\x02\u{80}\u{81}\x03\x02\x02\x02\u{81}\
	\x0b\x03\x02\x02\x02\u{82}\u{80}\x03\x02\x02\x02\u{83}\u{84}\x05\x40\x21\
	\x02\u{84}\u{85}\x07\x0f\x02\x02\u{85}\u{86}\x05\x0e\x08\x02\u{86}\x0d\x03\
	\x02\x02\x02\u{87}\u{89}\x0a\x04\x02\x02\u{88}\u{87}\x03\x02\x02\x02\u{89}\
	\u{8c}\x03\x02\x02\x02\u{8a}\u{88}\x03\x02\x02\x02\u{8a}\u{8b}\x03\x02\x02\
	\x02\u{8b}\x0f\x03\x02\x02\x02\u{8c}\u{8a}\x03\x02\x02\x02\u{8d}\u{8e}\x06\
	\x09\x02\x03\u{8e}\u{8f}\x05\x12\x0a\x02\u{8f}\u{90}\x07\x20\x02\x02\u{90}\
	\u{91}\x08\x09\x01\x02\u{91}\u{9d}\x03\x02\x02\x02\u{92}\u{93}\x06\x09\x03\
	\x03\u{93}\u{94}\x05\x14\x0b\x02\u{94}\u{95}\x07\x20\x02\x02\u{95}\u{96}\
	\x08\x09\x01\x02\u{96}\u{9d}\x03\x02\x02\x02\u{97}\u{98}\x06\x09\x04\x03\
	\u{98}\u{99}\x05\x16\x0c\x02\u{99}\u{9a}\x07\x20\x02\x02\u{9a}\u{9b}\x08\
	\x09\x01\x02\u{9b}\u{9d}\x03\x02\x02\x02\u{9c}\u{8d}\x03\x02\x02\x02\u{9c}\
	\u{92}\x03\x02\x02\x02\u{9c}\u{97}\x03\x02\x02\x02\u{9d}\u{9e}\x03\x02\x02\
	\x02\u{9e}\u{9c}\x03\x02\x02\x02\u{9e}\u{9f}\x03\x02\x02\x02\u{9f}\x11\x03\
	\x02\x02\x02\u{a0}\u{a1}\x05\x26\x14\x02\u{a1}\u{a2}\x07\x18\x02\x02\u{a2}\
	\u{a3}\x05\x40\x21\x02\u{a3}\u{a4}\x07\x03\x02\x02\u{a4}\u{a5}\x07\x1f\x02\
	\x02\u{a5}\u{a6}\x05\x40\x21\x02\u{a6}\u{a7}\x07\x07\x02\x02\u{a7}\u{a8}\
	\x05\x42\x22\x02\u{a8}\x13\x03\x02\x02\x02\u{a9}\u{aa}\x05\x26\x14\x02\u{aa}\
	\u{ab}\x07\x18\x02\x02\u{ab}\u{ac}\x05\x40\x21\x02\u{ac}\u{ad}\x07\x04\x02\
	\x02\u{ad}\u{ae}\x07\x1f\x02\x02\u{ae}\u{af}\x05\x40\x21\x02\u{af}\u{b0}\
	\x05\x18\x0d\x02\u{b0}\u{b1}\x05\x42\x22\x02\u{b1}\x15\x03\x02\x02\x02\u{b2}\
	\u{b3}\x05\x26\x14\x02\u{b3}\u{b4}\x07\x18\x02\x02\u{b4}\u{b5}\x05\x40\x21\
	\x02\u{b5}\u{b6}\x07\x06\x02\x02\u{b6}\u{b7}\x07\x1f\x02\x02\u{b7}\u{b8}\
	\x05\x40\x21\x02\u{b8}\u{b9}\x05\x20\x11\x02\u{b9}\u{ba}\x05\x42\x22\x02\
	\u{ba}\x17\x03\x02\x02\x02\u{bb}\u{bc}\x07\x05\x02\x02\u{bc}\u{bd}\x07\x1f\
	\x02\x02\u{bd}\u{be}\x05\x1a\x0e\x02\u{be}\u{bf}\x07\x1d\x02\x02\u{bf}\u{c2}\
	\x05\x1c\x0f\x02\u{c0}\u{c1}\x07\x1d\x02\x02\u{c1}\u{c3}\x05\x1e\x10\x02\
	\u{c2}\u{c0}\x03\x02\x02\x02\u{c2}\u{c3}\x03\x02\x02\x02\u{c3}\x19\x03\x02\
	\x02\x02\u{c4}\u{c5}\x09\x05\x02\x02\u{c5}\x1b\x03\x02\x02\x02\u{c6}\u{c7}\
	\x09\x05\x02\x02\u{c7}\x1d\x03\x02\x02\x02\u{c8}\u{c9}\x09\x05\x02\x02\u{c9}\
	\x1f\x03\x02\x02\x02\u{ca}\u{cb}\x08\x11\x01\x02\u{cb}\u{cc}\x05\x22\x12\
	\x02\u{cc}\u{d5}\x03\x02\x02\x02\u{cd}\u{ce}\x0c\x03\x02\x02\u{ce}\u{cf}\
	\x05\x42\x22\x02\u{cf}\u{d0}\x07\x1d\x02\x02\u{d0}\u{d1}\x05\x42\x22\x02\
	\u{d1}\u{d2}\x05\x22\x12\x02\u{d2}\u{d4}\x03\x02\x02\x02\u{d3}\u{cd}\x03\
	\x02\x02\x02\u{d4}\u{d7}\x03\x02\x02\x02\u{d5}\u{d3}\x03\x02\x02\x02\u{d5}\
	\u{d6}\x03\x02\x02\x02\u{d6}\x21\x03\x02\x02\x02\u{d7}\u{d5}\x03\x02\x02\
	\x02\u{d8}\u{dd}\x07\x0d\x02\x02\u{d9}\u{da}\x07\x1f\x02\x02\u{da}\u{dc}\
	\x09\x06\x02\x02\u{db}\u{d9}\x03\x02\x02\x02\u{dc}\u{df}\x03\x02\x02\x02\
	\u{dd}\u{db}\x03\x02\x02\x02\u{dd}\u{de}\x03\x02\x02\x02\u{de}\x23\x03\x02\
	\x02\x02\u{df}\u{dd}\x03\x02\x02\x02\u{e0}\u{e1}\x05\x26\x14\x02\u{e1}\u{e2}\
	\x05\x28\x15\x02\u{e2}\u{e3}\x07\x20\x02\x02\u{e3}\x25\x03\x02\x02\x02\u{e4}\
	\u{e6}\x09\x07\x02\x02\u{e5}\u{e4}\x03\x02\x02\x02\u{e6}\u{e7}\x03\x02\x02\
	\x02\u{e7}\u{e5}\x03\x02\x02\x02\u{e7}\u{e8}\x03\x02\x02\x02\u{e8}\x27\x03\
	\x02\x02\x02\u{e9}\u{ea}\x07\x1e\x02\x02\u{ea}\u{eb}\x07\x1b\x02\x02\u{eb}\
	\u{ec}\x05\x0e\x08\x02\u{ec}\x29\x03\x02\x02\x02\u{ed}\u{ef}\x05\x2c\x17\
	\x02\u{ee}\u{ed}\x03\x02\x02\x02\u{ef}\u{f0}\x03\x02\x02\x02\u{f0}\u{ee}\
	\x03\x02\x02\x02\u{f0}\u{f1}\x03\x02\x02\x02\u{f1}\u{f4}\x03\x02\x02\x02\
	\u{f2}\u{f5}\x05\x2c\x17\x02\u{f3}\u{f5}\x05\x2e\x18\x02\u{f4}\u{f2}\x03\
	\x02\x02\x02\u{f4}\u{f3}\x03\x02\x02\x02\u{f5}\x2b\x03\x02\x02\x02\u{f6}\
	\u{f7}\x05\x26\x14\x02\u{f7}\u{f8}\x05\x3a\x1e\x02\u{f8}\u{f9}\x05\x40\x21\
	\x02\u{f9}\u{fb}\x05\x3c\x1f\x02\u{fa}\u{fc}\x05\x30\x19\x02\u{fb}\u{fa}\
	\x03\x02\x02\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\u{ff}\x03\x02\x02\x02\
	\u{fd}\u{100}\x05\x32\x1a\x02\u{fe}\u{100}\x05\x42\x22\x02\u{ff}\u{fd}\x03\
	\x02\x02\x02\u{ff}\u{fe}\x03\x02\x02\x02\u{100}\u{101}\x03\x02\x02\x02\u{101}\
	\u{102}\x07\x20\x02\x02\u{102}\x2d\x03\x02\x02\x02\u{103}\u{104}\x05\x26\
	\x14\x02\u{104}\u{107}\x05\x3a\x1e\x02\u{105}\u{108}\x05\x32\x1a\x02\u{106}\
	\u{108}\x05\x42\x22\x02\u{107}\u{105}\x03\x02\x02\x02\u{107}\u{106}\x03\
	\x02\x02\x02\u{108}\u{109}\x03\x02\x02\x02\u{109}\u{10a}\x07\x20\x02\x02\
	\u{10a}\x2f\x03\x02\x02\x02\u{10b}\u{10c}\x05\x40\x21\x02\u{10c}\u{10e}\
	\x05\x3e\x20\x02\u{10d}\u{10f}\x05\x34\x1b\x02\u{10e}\u{10d}\x03\x02\x02\
	\x02\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\x31\x03\x02\x02\x02\u{110}\u{111}\
	\x05\x42\x22\x02\u{111}\u{112}\x05\x28\x15\x02\u{112}\x33\x03\x02\x02\x02\
	\u{113}\u{119}\x05\x36\x1c\x02\u{114}\u{115}\x05\x36\x1c\x02\u{115}\u{116}\
	\x05\x38\x1d\x02\u{116}\u{119}\x03\x02\x02\x02\u{117}\u{119}\x05\x38\x1d\
	\x02\u{118}\u{113}\x03\x02\x02\x02\u{118}\u{114}\x03\x02\x02\x02\u{118}\
	\u{117}\x03\x02\x02\x02\u{119}\x35\x03\x02\x02\x02\u{11a}\u{11b}\x05\x40\
	\x21\x02\u{11b}\u{11c}\x07\x12\x02\x02\u{11c}\u{11d}\x05\x42\x22\x02\u{11d}\
	\u{11e}\x05\x3c\x1f\x02\u{11e}\u{11f}\x05\x40\x21\x02\u{11f}\u{120}\x05\
	\x3e\x20\x02\u{120}\u{121}\x05\x42\x22\x02\u{121}\u{122}\x07\x13\x02\x02\
	\u{122}\x37\x03\x02\x02\x02\u{123}\u{124}\x05\x40\x21\x02\u{124}\u{125}\
	\x09\x08\x02\x02\u{125}\u{126}\x05\x40\x21\x02\u{126}\u{127}\x05\x3c\x1f\
	\x02\u{127}\u{128}\x05\x40\x21\x02\u{128}\u{129}\x05\x3e\x20\x02\u{129}\
	\x39\x03\x02\x02\x02\u{12a}\u{12f}\x07\x0d\x02\x02\u{12b}\u{12c}\x07\x1f\
	\x02\x02\u{12c}\u{12e}\x09\x06\x02\x02\u{12d}\u{12b}\x03\x02\x02\x02\u{12e}\
	\u{131}\x03\x02\x02\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{12f}\u{130}\x03\
	\x02\x02\x02\u{130}\x3b\x03\x02\x02\x02\u{131}\u{12f}\x03\x02\x02\x02\u{132}\
	\u{133}\x09\x05\x02\x02\u{133}\x3d\x03\x02\x02\x02\u{134}\u{135}\x07\x0d\
	\x02\x02\u{135}\x3f\x03\x02\x02\x02\u{136}\u{138}\x09\x07\x02\x02\u{137}\
	\u{136}\x03\x02\x02\x02\u{138}\u{139}\x03\x02\x02\x02\u{139}\u{137}\x03\
	\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\x41\x03\x02\x02\x02\u{13b}\
	\u{13d}\x09\x07\x02\x02\u{13c}\u{13b}\x03\x02\x02\x02\u{13d}\u{140}\x03\
	\x02\x02\x02\u{13e}\u{13c}\x03\x02\x02\x02\u{13e}\u{13f}\x03\x02\x02\x02\
	\u{13f}\x43\x03\x02\x02\x02\u{140}\u{13e}\x03\x02\x02\x02\u{141}\u{142}\
	\x05\x42\x22\x02\u{142}\u{143}\x07\x20\x02\x02\u{143}\x45\x03\x02\x02\x02\
	\x1c\x49\x50\x56\x5c\x64\x68\x6c\x71\u{80}\u{8a}\u{9c}\u{9e}\u{c2}\u{d5}\
	\u{dd}\u{e7}\u{f0}\u{f4}\u{fb}\u{ff}\u{107}\u{10e}\u{118}\u{12f}\u{139}\
	\u{13e}";

