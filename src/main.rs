use openssl::encrypt::{Decrypter, Encrypter};
use openssl::pkey::{PKey, Private};
use openssl::rsa::{Padding, Rsa};

fn gen(key_bit_len: u32) -> PKey<Private> {
    let keypair = Rsa::generate(key_bit_len).unwrap();
    PKey::from_rsa(keypair).unwrap()
}

fn main() {
    let keypair = gen(2048);
    let data = b"Hello, world!";
    // Encrypt the data with RSA PKCS1
    let mut encrypter = Encrypter::new(&keypair).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1_OAEP).unwrap();
    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(data).unwrap();
    let mut encrypted = vec![0; buffer_len];
    // Encrypt and truncate the buffer
    let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
    encrypted.truncate(encrypted_len);

    // Decrypt the data
    let mut decrypter = Decrypter::new(&keypair).unwrap();
    decrypter.set_rsa_padding(Padding::PKCS1_OAEP).unwrap();
    // Create an output buffer
    let buffer_len = decrypter.decrypt_len(&encrypted).unwrap();
    let mut decrypted = vec![0; buffer_len];
    // Encrypt and truncate the buffer
    let decrypted_len = decrypter.decrypt(&encrypted, &mut decrypted).unwrap();
    decrypted.truncate(decrypted_len);
    let plaintext = String::from_utf8(decrypted).unwrap();

    println!("Length: {}, plaintext: {}", decrypted_len, plaintext);
}
