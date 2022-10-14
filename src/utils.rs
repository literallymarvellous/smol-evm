use hex::FromHexError;

pub fn str_to_vec(s: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(s)
}