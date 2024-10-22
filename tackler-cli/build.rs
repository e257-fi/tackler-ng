/*
 * Copyright 2022-2024 E257.FI
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
use std::env;
use std::process::Command;

fn main() {
    let pkg_version = env!("CARGO_PKG_VERSION");

    #[allow(clippy::useless_format)]
    let version = match git_version() {
        Some(git_ver) => format!("{pkg_version} ({git_ver})"),
        None => format!("{pkg_version}"),
    };
    println!("cargo:rustc-env=VERSION={version}");
}

fn git_version() -> Option<String> {
    let tag: Option<String> = Command::new("git")
        .args(["describe", "--exact-match"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|tag| {
            let t = tag.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.into())
            }
        });

    let commit_id: Option<String> = Command::new("git")
        .args(["rev-parse", "--short=15", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|id| id.trim().into());

    let status: Option<String> = Command::new("git")
        .args(["status", "--short", "--porcelain"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|stats| {
            if stats.trim().is_empty() {
                None
            } else {
                Some(":modified".to_string())
            }
        });

    match commit_id {
        Some(id) => match status {
            Some(s) => Some(id + &s),
            None => match tag {
                // Show tag only when the modified status is clean
                Some(t) => Some(t + " - " + &id),
                None => Some(id),
            },
        },
        None => None,
    }
}
