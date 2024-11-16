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
use std::fmt::{Debug, Formatter, Write};
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

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hash: {:?}", self.hash_algo)
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
            "SHA3-256" => Ok(Hash {
                hash_algo: "SHA3-256".to_string(),
                hasher: Box::new(sha3::Sha3_256::default()),
            }),
            "SHA3-512" => Ok(Hash {
                hash_algo: "SHA3-512".to_string(),
                hasher: Box::new(sha3::Sha3_512::default()),
            }),
            _ => {
                let msg = format!("Unsupported hash algorithm: '{algo}'");
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
    fn hasher_no_input() {
        let hash = Hash::from("SHA-256").unwrap(/*:test:*/);

        let uuids = vec![];
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        // same as:
        //   echo -ne "" | sha256sum
        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn hasher_empty_input() {
        let hash = Hash::from("SHA-256").unwrap(/*:test:*/);

        let uuids = vec!["".to_string()];
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        // same as:
        //   echo -ne "\n" | sha256sum
        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"
        );
    }

    #[test]
    fn hasher_multiple_rounds() {
        let hash = Hash::from("SHA-256").unwrap(/*:test:*/);

        let foo = vec!["foo".to_string()];
        let cs_foo = hash.checksum(&foo, "\n".as_bytes());

        assert!(cs_foo.is_ok());
        assert_eq!(
            cs_foo.unwrap(/*:test:*/).value,
            "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"
        );

        let bar = vec!["bar".to_string()];
        let cs_bar = hash.checksum(&bar, "\n".as_bytes());
        assert!(cs_bar.is_ok());
        assert_eq!(
            cs_bar.unwrap(/*:test:*/).value,
            "7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730"
        );
    }

    #[test]
    fn hasher_err() {
        let hash = Hash::from("foo");

        assert!(hash.is_err());
        assert_eq!(
            hash.err().unwrap(/*:test:*/).to_string(),
            "Unsupported hash algorithm: 'foo'".to_string()
        );
    }

    //
    // Tests of the correct selection of Hash Algorithm
    //
    fn get_test_vector() -> Vec<String> {
        vec![
            "9c123cbe-4acd-475d-bbcf-96c1fcba58cb".to_string(),
            "2e546b18-6ce6-4bb3-9f4b-21b77a768a4c".to_string(),
            "67bdab27-da08-4647-b0d1-57c9ed129657".to_string(),
        ]
    }

    #[test]
    fn hasher_sha2_256() {
        let hash = Hash::from("SHA-256").unwrap(/*:test:*/);

        let uuids = get_test_vector();
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "16418783ef294f830721159ee59cc3388c8b69c13afba2256cf756c6097fe687"
        );
    }
    #[test]
    fn hasher_sha2_512() {
        let hash = Hash::from("SHA-512").unwrap(/*:test:*/);

        let uuids = get_test_vector();
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "51a370f86b7218012c8a7a555a1ae099b32fe83ed032e82481de5fa7ea3a90baa6948a4f559668ad3696a08b0445fe4e5964dba695b45653b4e678ab200ede17"
        );
    }

    #[test]
    fn hasher_sha2_512_256() {
        let hash = Hash::from("SHA-512/256").unwrap(/*:test:*/);

        let uuids = get_test_vector();
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "fd5e794d47589c83ed5def2485699f232aed7e7df2869ef8b681cbf07af7cb66"
        );
    }

    #[test]
    fn hasher_sha3_256() {
        let hash = Hash::from("SHA3-256").unwrap(/*:test:*/);

        let uuids = get_test_vector();
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "ee677c3486b0b6c8d61eb3b5cb650762dbf741bb6b050934f4c3c4f6551d1841"
        );
    }

    #[test]
    fn hasher_sha3_512() {
        let hash = Hash::from("SHA3-512").unwrap(/*:test:*/);

        let uuids = get_test_vector();
        let cs = hash.checksum(&uuids, "\n".as_bytes());

        assert!(cs.is_ok());
        assert_eq!(
            cs.unwrap(/*:test:*/).value,
            "6cc3e6d3eb1d920ac3439a0b748244aa5f997ba0a813457bae7b99f3f58f035ce18eb3646a35517c19987dfe0bb60ebcc81ee5320f4d2348132fdb55bb8d8a25"
        );
    }
}
