/*
 * Copyright 2025 E257.FI
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

use criterion::{criterion_group, criterion_main, Criterion};
use indoc::indoc;
use tackler_core::kernel::Settings;
use tackler_core::parser::string_to_txns;
use tackler_rs::IndocUtils;

fn criterion_benchmark(c: &mut Criterion) {
    let mut settings = Settings::default();

    #[rustfmt::skip]
    let input = 
        indoc!(
            "|2024-12-31T23:58:59.123456789+02:00 (#001) 'bells 'n whistles
             | # uuid: 506a2d55-2375-4d51-af3a-cf5021f04de9
             | # tags: cef, first, second
             | # location: geo:1.111,2.222,3.333
             | ; first txn comment
             | ; second txn comment
             | e:d:f 26 bar·He_50L @ 1.25 EUR ; 32.50 EUR
             | a:b:c
             |
             |").strip_margin();

    c.bench_function("parser", |b| {
        b.iter(|| {
            let res = string_to_txns(&mut input.as_str(), &mut settings);
            assert!(res.is_ok());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
