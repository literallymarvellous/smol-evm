use bytes::{BytesMut, BufMut};
use ethnum::U256;
use hex::FromHexError;
use crate::ExecutionContext;

pub fn str_to_vec(s: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(s)
}

// for testing purposes
pub fn build_context(str: &str) -> ExecutionContext {
    let bytecode = str_to_vec(str).unwrap();
    ExecutionContext::new(bytecode)
}

pub fn vec_u8_to_u256(v: &[u8]) -> U256 {
    // create bytes32 array ie. [0,...32]
    let mut bytes = BytesMut::new();
    let bytes32 = [0u8; 32];
    bytes.put(&bytes32[..]);

    let range = v.len();
    // copy slice into bytes32 array 
    // v: [255, 255] -> bytes32: [0, ..., 255, 255]
    bytes[32 - range..32].copy_from_slice(v);

    let bytes_v = bytes.to_vec().try_into().unwrap();
    U256::from_be_bytes(bytes_v)
}