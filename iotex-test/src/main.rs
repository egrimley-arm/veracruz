use psa_crypto::operations::cipher::encrypt;
use psa_crypto::operations::key_management::import;
use psa_crypto::types::algorithm::Cipher::Ctr;
use psa_crypto::types::key::{Attributes, Lifetime, Policy, Type, UsageFlags};
use std::fs::read;
use std::io::{self, Read, Write};

fn main() {
    psa_crypto::init().unwrap();
    let mut raw_key = read("key").unwrap();
    raw_key.resize(16, 0);
    let mut plain = Vec::new();
    io::stdin().read_to_end(&mut plain).unwrap();
    let mut cypher = vec![0; plain.len()];
    let iv = vec![0; 16];
    let mut usage_flags: UsageFlags = Default::default();
    usage_flags.set_decrypt();
    usage_flags.set_encrypt();
    let attributes = Attributes {
        key_type: Type::Aes,
        bits: 128,
        lifetime: Lifetime::Volatile,
        policy: Policy {
            usage_flags,
            permitted_algorithms: Ctr.into(),
        },
    };
    let key = import(attributes, None, &raw_key).unwrap();
    encrypt(key, Ctr, &plain, &iv, &mut cypher).unwrap();
    io::stdout().write_all(&cypher).unwrap();
}
