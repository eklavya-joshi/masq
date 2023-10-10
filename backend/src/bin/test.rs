use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    SalsaBox, PublicKey, SecretKey
};

fn main() {
    let a_private_key = SecretKey::generate(&mut OsRng);
    let a_public_key = a_private_key.public_key();

    let b_private_key = SecretKey::generate(&mut OsRng);
    let b_public_key = b_private_key.public_key();

    let a_box = SalsaBox::new(&b_public_key, &a_private_key);

    let nonce = SalsaBox::generate_nonce(&mut OsRng);

    let plaintext = b"I love poop";

    let ciphertext = a_box.encrypt(&nonce, &plaintext[..]).unwrap();

    let b_box = SalsaBox::new(&a_public_key, &b_private_key);

    let decrypted_plaintext = b_box.decrypt(&nonce, &ciphertext[..]).unwrap();

    println!("{}", String::from_utf8(decrypted_plaintext).unwrap());
}