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

pub trait Text: std::fmt::Debug {
    /// Get metadata item as text
    fn text(&self) -> Vec<String>;
}

#[derive(Debug)]
pub enum MetadataItem {
    TxnSetChecksum(TxnSetChecksum),
    GitInputReference(GitInputReference)
}

impl Text for MetadataItem {
    fn text(&self) -> Vec<String> {
        match self {
            Self::GitInputReference(gif) => {
                gif.text()
            },
            Self::TxnSetChecksum(tscs) => {
                tscs.text()
            }
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    // todo: fid pub access
    pub items: Vec<MetadataItem>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata { items: Vec::new() }
    }
    pub fn text(&self) -> String {
        let ts = self.items
            .iter()
            .flat_map(|item| {
                let mut vs = item.text();
                // put a newline between metadata items
                vs.push(String::default());
                vs
            })
            .collect::<Vec<String>>();
        ts.join("\n")
    }
}

/// Generic checksum value
#[derive(Debug)]
pub struct Checksum {
    /// used hash algorithm
    pub algorithm: String,
    /// hexadecimal hash value
    pub value: String,
}

/// Txn Set Checksum metadata item
#[derive(Debug)]
pub struct TxnSetChecksum {
    /// size of transaction set
    pub size: usize,
    /// hash of Txn Set Checksum
    pub hash: Checksum,
}
impl Text for TxnSetChecksum {
    fn text(&self) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = 15;
        vec![
            format!("Txn Set Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
            format!("{:>pad$} : {}", "Set size", self.size),
        ]
    }
}

#[derive(Debug)]
pub struct GitInputReference {
    pub commit: String,
    pub reference: Option<String>,
    pub dir: String,
    pub suffix: String,
    pub message: String,
}

impl Text for GitInputReference {
    /*
        Seq(
          "Git storage:",
          "  commit:  " + commit,
          "  ref:     " + ref.getOrElse("FIXED by commit"),
          "  dir:     " + dir,
          "  suffix:  " + suffix,
          "  message: " + message,
        )
     */
    fn text(&self) -> Vec<String> {
        let pad = 15;
        vec![
            format!("Git Storage"),
            format!("{:>pad$} : {}", "commit", self.commit),
            format!("{:>pad$} : {}", "reference", self.reference.as_ref().unwrap_or(&"FIXED by commit - no ref!".to_string())),
            format!("{:>pad$} : {}", "directory", self.dir),
            format!("{:>pad$} : {}", "suffix", self.suffix),
            format!("{:>pad$} : {}", "message", self.message.trim()),
        ]
    }
}
