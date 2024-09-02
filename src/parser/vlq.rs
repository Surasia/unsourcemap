use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VLQError {
    #[error("Invalid character in VLQ")]
    InvalidCharacter,
    #[error("Incomplete VLQ sequence")]
    IncompleteSequence,
}

/// Decodes a Variable-Length Quantity (VLQ) encoded string into a vector of integers.
///
/// Reads the input string character by character, interpreting each character
/// as a Base64 digit. It then decodes these digits into a series of integer values.
///
/// 1. Each character is converted to its corresponding Base64 value.
/// 2. The lower 5 bits of each value are used to build up the result.
/// 3. If the 6th bit (0x20) is set, it indicates that more digits follow for the current number.
/// 4. When a digit with the 6th bit clear is encountered, it signals the end of the current number.
/// 5. The accumulated value is then processed:
///    - The least significant bit determines the sign (1 for negative, 0 for positive).
///    - The value is right-shifted by 1 to remove the sign bit.
///    - If the sign bit was 1, the value is negated.
/// 6. The resulting integer is added to the output vector.
/// 7. This process repeats for each number in the input string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the VLQ encoded data.
///
/// # Returns
///
/// * `Result<Vec<i32>>` - A Result containing a vector of decoded integers if successful,
///                        or an error if the input is invalid or incomplete.
///
/// # Errors
///
/// This function will return an error if:
/// * An invalid character (not in the Base64 set) is encountered.
/// * The input ends with an incomplete sequence (i.e., the last digit has its 6th bit set).
pub fn decode_vlq(input: &str) -> Result<Vec<i32>> {
    const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = Vec::new();
    let mut value = 0;
    let mut shift = 0;

    for c in input.chars() {
        let digit = BASE64_CHARS.find(c).ok_or(VLQError::InvalidCharacter)? as i32;

        value |= (digit & 31) << shift;
        shift += 5;

        if digit & 32 == 0 {
            let mut final_value = value >> 1;
            if value & 1 != 0 {
                final_value = -final_value;
            }
            result.push(final_value);
            value = 0;
            shift = 0;
        }
    }

    if shift != 0 {
        return Err(VLQError::IncompleteSequence.into());
    }

    Ok(result)
}
