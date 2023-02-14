/*
 * Copyright 2022 E257.FI
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
use antlr_rust::token_stream::TokenStream;
use antlr_rust::BailErrorStrategy;
use antlr_rust::TidAble;

use txnlexer::LocalTokenFactory;
use txnparser::TxnParser;
use txnparser::TxnParserContextType;

pub mod txnlexer;
pub mod txnparser;
pub mod txnparserlistener;

impl<'input, I> TxnParser<'input, I, BailErrorStrategy<'input, TxnParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, BailErrorStrategy::new())
    }
}
