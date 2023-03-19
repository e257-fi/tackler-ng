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

use crate::kernel::accumulator;
use crate::kernel::report_item_selector::{RegisterByAccountSelector, RegisterItemSelector};
use crate::model::{RegisterEntry, TxnData};
use crate::report::Report;
use std::io;
use std::io::Write;

pub struct RegisterReporter {
    title: String,
}

fn re_fmt<W: Write + ?Sized>(f: &mut Box<W>, re: &RegisterEntry) {
    if !re.posts.is_empty() {
        write!(f, "{}", re).unwrap();
    }
}

impl Report for RegisterReporter {
    fn write_txt_report(txns: &TxnData) {
        let ras = RegisterByAccountSelector::from(&["^a:b$", "^e:.*"]).unwrap();

        let mut w: Box<dyn Write> = Box::new(io::stdout());
        //accumulator::register_engine(&txns.txns, &mut w , re_fmt);
        accumulator::register_engine(&txns.txns, &ras, &mut w, |f, re| {
            if !re.posts.is_empty() {
                write!(f, "{}", re).unwrap();
            }
        });
    }
}
