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
pub use crate::parser::tackler_txns::git_to_txns;
pub use crate::parser::tackler_txns::paths_to_txns;
pub use crate::parser::tackler_txns::string_to_txns;
pub use crate::parser::tackler_txns::GitInputSelector;
use winnow::error::{ErrMode, FromExternalError};

mod error;
mod tackler_parser;
mod tackler_txns;

use crate::kernel::settings::Settings;
use crate::parser::error::TacklerTxnError;
use winnow::Stateful;

pub(crate) mod parts;

pub(crate) type Stream<'is> = Stateful<&'is str, &'is mut Settings>;

pub(crate) fn make_semantic_error<
    'is,
    E: winnow::error::FromExternalError<Stream<'is>, TacklerTxnError>,
>(
    is: &mut Stream<'is>,
    msg: &str,
) -> ErrMode<E> {
    winnow::error::ErrMode::from_external_error(
        is,
        winnow::error::ErrorKind::Verify,
        TacklerTxnError::semantic_error(msg),
    )
    .cut()
}

pub(crate) fn from_error<
    'is,
    E: winnow::error::FromExternalError<Stream<'is>, TacklerTxnError>,
    SE: std::error::Error + ?Sized,
>(
    is: &mut Stream<'is>,
    err: &SE,
) -> ErrMode<E> {
    winnow::error::ErrMode::from_external_error(
        is,
        winnow::error::ErrorKind::Verify,
        TacklerTxnError::semantic_error(err.to_string().as_str()),
    )
    .cut()
}

/*
 * TODO: This logic should be 1:1 with TxnLexer.g4
 *       (ID, SUBID and NameChar + NameStartChar)
 * Real account names are coming through parser+lexer,
 * So these are validating Chart-of-Xyz config/settings
 * entries (accounts, tags, commodities).
 * E.g. checking these is a nicety for user
 * (warn about invalid Chart-Of-Xyz).
 */
#[inline]
fn illegal_characters(c: char) -> bool {
    c == ':' || c.is_whitespace()
}

// todo: this is too relaxed
fn is_valid_id_start_char(c: char) -> bool {
    !(c.is_ascii_digit() || c == ':' || c == '-' || c == '_' || c == '·' || c.is_whitespace())
}

// this is fine, once is_valid_id_start_char is fixed
fn is_valid_sub_id_start_char(c: char) -> bool {
    c.is_numeric() || is_valid_id_start_char(c)
}

// todo: this is too relaxed
pub fn is_valid_id(token: &str) -> bool {
    !token.is_empty()
        && token.starts_with(is_valid_id_start_char)
        && !token.contains(illegal_characters)
}

// todo: this is too relaxed
pub fn is_valid_sub_id(token: &str) -> bool {
    !token.is_empty()
        && token.starts_with(is_valid_sub_id_start_char)
        && !token.contains(illegal_characters)
}

#[cfg(test)]
mod tests;
