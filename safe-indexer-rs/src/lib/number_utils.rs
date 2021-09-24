use anyhow::Result;

pub fn to_hex_string(input: u64) -> Result<String> {
    Ok(format!("0x{:x}", input))
}

pub fn from_hex_string(input: &str) -> Result<u64> {
    let result = &input[2..input.len()];
    Ok(u64::from_str_radix(&result, 16)?)
}
