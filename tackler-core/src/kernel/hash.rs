/*
 * Copyright 2023-2024 E257.FI
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

use digest::DynDigest;
use std::error::Error;
use std::fmt::Write;
use tackler_api::metadata::Checksum;

#[derive(Clone)]
pub struct Hash {
    hash_algo: String,
    hasher: Box<dyn DynDigest>,
}

impl Default for Hash {
    fn default() -> Self {
        Hash {
            hash_algo: "SHA-256".to_string(),
            hasher: Box::new(sha2::Sha256::default()),
        }
    }
}

impl Hash {
    pub fn from(algo: &str) -> Result<Hash, Box<dyn Error>> {
        match algo {
            "SHA-256" => Ok(Hash {
                hash_algo: "SHA-256".to_string(),
                hasher: Box::new(sha2::Sha256::default()),
            }),
            "SHA-512" => Ok(Hash {
                hash_algo: "SHA-512".to_string(),
                hasher: Box::new(sha2::Sha512::default()),
            }),
            "SHA-512/256" => Ok(Hash {
                hash_algo: "SHA-512/256".to_string(),
                hasher: Box::new(sha2::Sha512_256::default()),
            }),
            _ => {
                let msg = format!("Unsupported hash algorithm: {algo}");
                Err(msg.into())
            }
        }
    }

    pub fn checksum(&self, items: &[String], separator: &[u8]) -> Result<Checksum, Box<dyn Error>> {
        let mut hasher = self.hasher.clone();

        for i in items {
            hasher.update(i.as_bytes());
            hasher.update(separator);
        }
        let hash = hasher.finalize();

        Ok(Checksum {
            algorithm: self.hash_algo.clone(),
            value: hash.iter().fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02x}");
                output
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher() {
        let uuids = vec![
            "9c123cbe-4acd-475d-bbcf-96c1fcba58cb".to_string(),
            "2e546b18-6ce6-4bb3-9f4b-21b77a768a4c".to_string(),
            "67bdab27-da08-4647-b0d1-57c9ed129657".to_string(),
        ];
        let hash = Hash::from("SHA-256").unwrap(/*:test:*/);
        let cs = hash.checksum(&uuids, "\n".as_bytes()).unwrap(/*:test:*/);
        assert_eq!(
            cs.value,
            "16418783ef294f830721159ee59cc3388c8b69c13afba2256cf756c6097fe687"
        );
    }
    #[test]
    fn hasher_err() {
        let hash = Hash {
            hash_algo: "foo".to_string(),
            hasher: Box::new(sha2::Sha512_256::default()),
        };

        assert_eq!(hash.hash_algo, "foo".to_string());
    }
}
