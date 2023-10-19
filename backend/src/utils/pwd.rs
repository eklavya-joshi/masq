use bcrypt::hash_with_salt;
use rand::Rng;

pub struct Pwd {
    pub salt: String,
    pub hash: String,
}

pub async fn encrypt(str: &str) -> Pwd {
    let salt = rand::thread_rng().gen::<[u8; 16]>();
    let hash = hash_with_salt(str, bcrypt::DEFAULT_COST, salt)
        .unwrap()
        .to_string();

    let salt = hex::encode(salt);

    Pwd { salt, hash }
}

pub async fn decrypt(salt: &str, hash: &str, str: &str) -> bool {
    let salt = match hex::decode(salt) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let salt = match salt.try_into() {
        Ok(x) => x,
        Err(_) => return false,
    };

    let new_hash = hash_with_salt(str, bcrypt::DEFAULT_COST, salt)
        .unwrap()
        .to_string();

    hash.eq(&new_hash)
}
