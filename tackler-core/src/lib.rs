/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]

pub mod config;
pub mod export;
pub mod filter;
pub mod kernel;
pub mod math;
pub mod model;
pub mod parser;
pub mod report;

pub mod tackler {
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
}
