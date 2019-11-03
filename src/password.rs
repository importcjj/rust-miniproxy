use base64::{self, DecodeError};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn new() -> String {
    let mut rng = thread_rng();
    let mut password: Vec<u8> = (0..=255).collect();
    password.shuffle(&mut rng);

    base64::encode(&password)
}

pub fn decode_password(password: &str) -> Result<Vec<u8>, DecodeError> {
    base64::decode(password)
}
