use anyhow::Result;
use tiny_keccak::{Hasher, Keccak};

pub fn to_hex_string(input: u64) -> Result<String> {
    Ok(format!("0x{:x}", input))
}

pub fn from_hex_string(input: &str) -> Result<u64> {
    let result = if input.starts_with("0x") {
        &input[2..input.len()]
    } else {
        input
    };
    Ok(u64::from_str_radix(&result, 16)?)
}

pub fn to_decimal(input: &str) -> Result<String> {
    let decimal = from_hex_string(input)?;
    Ok(decimal.to_string())
}

pub fn keccak256<B>(data: B) -> [u8; 32]
where
    B: AsRef<[u8]>,
{
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data.as_ref());
    hasher.finalize(&mut output);
    output
}

pub fn keccak256_str(input: &str) -> String {
    let output = keccak256(input.as_bytes());
    format!("0x{}", hex::encode(output))
}
