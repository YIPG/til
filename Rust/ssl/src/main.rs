use aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

fn main() {
    let key = GenericArray::clone_from_slice(b"an example very very secret key.");
    let aead = Aes256Gcm::new(key);

    let nonce = GenericArray::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = aead
        .encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");
    let plaintext = aead
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    assert_eq!(&plaintext, b"plaintext message");
}
