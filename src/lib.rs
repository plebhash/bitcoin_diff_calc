use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use num_bigint::{BigUint, ToBigInt};
use num_traits::Num;

/// Converts a hash in hex format to a mining difficulty value.
///
/// # Arguments
///
/// * `hash_hex` - A `&str` representing the hash in hexadecimal format.
///
/// # Returns
///
/// Returns a `f64` representing the calculated difficulty for the provided hash.
pub fn difficulty_from_hash(hash_hex: &str) -> f64 {
    // Define the difficulty 1 target as a hexadecimal string
    let diff1_target_hex = "00000000FFFF0000000000000000000000000000000000000000000000000000";
    // Convert difficulty 1 target hex to BigUint
    let diff1_target = BigUint::from_str_radix(diff1_target_hex, 16).unwrap();
    // Convert the hash hex to BigUint
    let current_target = BigUint::from_str_radix(hash_hex, 16).unwrap();

    // Convert both values to BigDecimal for floating-point division
    let diff1_target_decimal = BigDecimal::from_biguint(diff1_target, 0);
    let current_target_decimal = BigDecimal::from_biguint(current_target, 0);

    // Calculate the difficulty as a BigDecimal
    let difficulty_decimal = &diff1_target_decimal / &current_target_decimal;

    // Convert the result to f64 and return
    difficulty_decimal.to_f64().unwrap()
}

/// Calculates mining difficulty based on hashrate and block time.
///
/// # Arguments
///
/// * `hashrate` - A `u128` representing the miner's hashrate (hashes per second).
/// * `time` - A `u128` representing the average time (in seconds) between blocks.
///
/// # Returns
///
/// Returns a `f64` representing the calculated difficulty based on the provided
/// hashrate and block time.
pub fn difficulty_from_hashrate_and_block_time(hashrate: u128, time: u128) -> f64 {
    ((time * hashrate) >> 32) as f64
}

/// Converts a difficulty value to a target, used in blockchain mining.
///
/// In blockchain systems, the "target" is a threshold value miners need to reach
/// for a block hash to be considered valid. The higher the difficulty, the lower
/// the target.
///
/// # Arguments
///
/// * `difficulty` - A `f64` representing the current mining difficulty.
///
/// # Returns
///
/// Returns a `BigUint` representing the target value that corresponds to the
/// given difficulty.
pub fn difficulty_to_target(difficulty: f64) -> BigUint {
    // Difficulty 1 target in hexadecimal
    let diff1_target_hex = "00000000FFFF0000000000000000000000000000000000000000000000000000";
    // Convert the hex string to BigUint
    let diff1_target = BigUint::from_str_radix(diff1_target_hex, 16).unwrap();
    // Convert diff1_target to BigDecimal with scale 0
    let diff1_target_decimal = BigDecimal::from_biguint(diff1_target, 0);
    // Convert difficulty to BigDecimal
    let difficulty_decimal = BigDecimal::from_f64(difficulty).unwrap();
    // Perform the division
    let target_decimal = &diff1_target_decimal / &difficulty_decimal;
    // Get the integer part of the target_decimal
    let target_bigint = target_decimal.with_scale(0).to_bigint().unwrap();
    // Convert to BigUint
    let target = target_bigint.to_biguint().unwrap();
    target
}

/// Converts a target value to a "compact" representation, often used in mining protocols.
///
/// The "compact" form, also known as "nBits" in some blockchain protocols, is a
/// 32-bit representation of a target threshold, combining an exponent and a
/// coefficient.
///
/// # Arguments
///
/// * `target` - A `BigUint` representing the target threshold value.
///
/// # Returns
///
/// Returns a `u32` representing the target in compact form.
pub fn target_to_compact(target: BigUint) -> u32 {
    // Convert target to bytes in big-endian order
    let mut target_bytes = target.to_bytes_be();
    // Remove leading zeros
    while !target_bytes.is_empty() && target_bytes[0] == 0 {
        target_bytes.remove(0);
    }
    let mut exponent = target_bytes.len() as u32;

    // Coefficient is the first 3 bytes
    let coefficient_bytes = if exponent <= 3 {
        // Pad with zeros if less than 3 bytes
        let mut temp = target_bytes.clone();
        while temp.len() < 3 {
            temp.push(0);
        }
        temp
    } else {
        target_bytes[0..3].to_vec()
    };

    let mut coefficient = ((coefficient_bytes[0] as u32) << 16)
        | ((coefficient_bytes[1] as u32) << 8)
        | (coefficient_bytes[2] as u32);

    // If the first byte is >= 0x80, shift right and increment exponent
    if coefficient_bytes[0] & 0x80 != 0 {
        coefficient >>= 8;
        exponent += 1;
    }

    // Combine exponent and coefficient into nBits
    let nbits = (exponent << 24) | (coefficient & 0x007FFFFF);
    nbits
}

/// Converts a difficulty value directly to a compact target representation.
///
/// This function combines the `difficulty_to_target` and `target_to_compact` functions
/// to convert a given difficulty into its compact target form, which is useful in
/// mining protocols.
///
/// # Arguments
///
/// * `difficulty` - A `f64` representing the current difficulty level.
///
/// # Returns
///
/// Returns a `u32` that represents the compact form of the calculated target.
pub fn difficulty_to_compact(difficulty: f64) -> u32 {
    let target = difficulty_to_target(difficulty);
    let nbits = target_to_compact(target);
    nbits
}