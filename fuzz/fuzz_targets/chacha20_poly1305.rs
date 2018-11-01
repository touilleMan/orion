#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate orion;
pub mod util;

use util::*;

fuzz_target!(|data: &[u8]| {
    let (key, nonce) = chacha_key_nonce_setup(12, data);
    let mut aad = Vec::new();
    apply_from_input_heap(&mut aad, data, key.len() + nonce.len());
    let mut plaintext = Vec::new();
    apply_from_input_heap(&mut plaintext, data, key.len() + nonce.len() + aad.len());

    let mut ciphertext_with_tag_orion: Vec<u8> = vec![0u8; plaintext.len() + 16];
    let mut plaintext_out_orion = vec![0u8; plaintext.len()];

    orion::hazardous::aead::ietf_chacha20_poly1305_encrypt(
        &key,
        &nonce,
        &plaintext,
        &aad,
        &mut ciphertext_with_tag_orion,
    ).unwrap();
    orion::hazardous::aead::ietf_chacha20_poly1305_decrypt(
        &key,
        &nonce,
        &ciphertext_with_tag_orion,
        &aad,
        &mut plaintext_out_orion,
    ).unwrap();
});
