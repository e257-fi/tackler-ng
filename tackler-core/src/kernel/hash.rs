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

use sha2::{Digest, Sha256, Sha512, Sha512_256};
use std::error::Error;
use tackler_api::Checksum;

#[derive(Debug, Clone)]
pub struct Hash {
    hash_algo: String,
    // todo: hasher instance
}

impl Default for Hash {
    fn default() -> Self {
        Hash {
            hash_algo: "SHA-256".to_string(),
        }
    }
}

impl Hash {
    pub fn from(algo: &str) -> Result<Hash, Box<dyn Error>> {
        // todo: SHA-256, SHA-512/256, SHA-512
        match algo {
            "SHA-256" => {
                //sha2::Sha256::default()
                Ok(Hash {
                    hash_algo: "SHA-256".to_string(),
                })
            }
            //"SHA-256" => Box::new(sha2::Sha256::default()),
            //"SHA-512/256" => Box::new(sha2::Sha512_256::default()),
            //"SHA-512" => Box::new(sha2::Sha512::default()),
            _ => {
                let msg = format!("Unsupported hash algorithm: {algo}");
                Err(msg.into())
            }
        }
    }

    pub fn checksum(
        &self,
        items: &Vec<String>,
        separator: &[u8],
    ) -> Result<Checksum, Box<dyn Error>> {
        //let mut hasher: Box<dyn DynDigest> = match hash_algo {
        let mut hasher = match self.hash_algo.as_str() {
            "SHA-256" => sha2::Sha256::default(),
            //"SHA-256" => Box::new(sha2::Sha256::default()),
            //"SHA-512/256" => Box::new(sha2::Sha512_256::default()),
            //"SHA-512" => Box::new(sha2::Sha512::default()),
            _ => {
                let msg = format!("Unsupported hash algorithm: {}", self.hash_algo);
                return Err(msg.into());
            }
        };

        hasher.reset();

        for i in items {
            hasher.update(i.as_bytes());
            hasher.update(separator);
        }
        let hash = hasher.finalize();

        Ok(Checksum {
            algorithm: self.hash_algo.clone(),
            value: format!("{:x}", hash),
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
        let hash = Hash::from("SHA-256").unwrap();
        let cs = hash.checksum(&uuids, "\n".as_bytes()).unwrap();
        assert_eq!(
            cs.value,
            "16418783ef294f830721159ee59cc3388c8b69c13afba2256cf756c6097fe687"
        );
    }
    #[test]
    fn hasher_err() {
        let hash = Hash {
            hash_algo: "foo".to_string(),
        };

        assert_eq!(hash.hash_algo, "foo".to_string());
    }
}
