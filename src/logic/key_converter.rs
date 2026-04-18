use hex;

pub fn get_key(key: [u8; 32]) -> String {
    let pub_key = hex::encode(key);
    pub_key
}
