use rand::Rng;
use bcrypt::hash_with_salt;

pub struct Crypt {
    pub salt: String,
    pub hash: String,
}

pub fn encrypt(str: String) -> Crypt {

    let salt = rand::thread_rng().gen::<[u8; 16]>();
    let hash = hash_with_salt(str, bcrypt::DEFAULT_COST, salt).unwrap().to_string();

    let salt = hex::encode(salt);

    Crypt {
        salt,
        hash
    }

}

pub fn decrypt(salt: String, hash: String, str: String) -> bool {

    let salt = hex::decode(salt).unwrap().try_into().unwrap();
    let new_hash = hash_with_salt(str, bcrypt::DEFAULT_COST, salt).unwrap().to_string();

    hash.eq(&new_hash)
}