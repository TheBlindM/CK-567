#![feature(array_chunks)]

use std::borrow::Borrow;

use aes::{Aes128, Aes128Dec, Aes128Enc};
use aes::cipher::{
    BlockCipher, BlockDecrypt, BlockEncrypt, generic_array::GenericArray,
    KeyInit,
};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut};
use aes::cipher::block_padding::Pkcs7;
use libaes::Cipher;
use rand::Rng;

pub const RANDOM_AES_KEY: &[u8] = b"abcdefghijklmnopqrstuvwxyz";


pub fn aesEncrypt(plaintext: Vec<u8>) -> (String,String, Vec<u8>) {
    let mut rng = rand::thread_rng();
    let key: String = (0..16)
        .map(|_| unsafe {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(RANDOM_AES_KEY.get_unchecked(idx).to_owned() )
        }).collect();

    let iv: String = (0..16)
        .map(|_| unsafe {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(RANDOM_AES_KEY.get_unchecked(idx).to_owned() )
        }).collect();

    let cipher = Cipher::new_128(key.as_bytes()[0..16].try_into().unwrap());
    return (key, iv.clone(),cipher.cbc_encrypt(iv.clone().as_ref(), &plaintext));
}

pub fn aesDecrypt(key: String, iv: String, ciphertext: String) -> Vec<u8> {
    let cipher = Cipher::new_128(&key.as_bytes()[0..16].try_into().unwrap());
    cipher.cbc_decrypt(iv.as_bytes(), ciphertext.as_bytes())
}

