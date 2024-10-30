use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "bitcoin_diff_calc",
    about = "a convenient toolkit for calculations on Bitcoin mining difficulty"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Calculate difficulty based on hashrate and average block time
    Difficulty(DifficultyArgs),
    /// Calculate difficulty based on some specific hash
    DifficultyFromHash(DifficultyFromHashArgs),
    /// Calculate compact target for some difficulty (as floating point)
    Compact(CompactArg),
}

#[derive(Parser, Debug)]
pub struct DifficultyArgs {
    #[arg(short = 'r', long, value_parser = parse_u128_with_underscores, help = "network hashrate (H/s)")]
    pub hashrate: u128,
    #[arg(short, long, value_parser = parse_u128_with_underscores, help = "average block time (s)")]
    pub block_time: u128,
}

#[derive(Parser, Debug)]
pub struct DifficultyFromHashArgs {
    #[arg(short = 's', long, value_parser = validate_hex_hash, help = "hash")]
    pub hash: String,
}

#[derive(Parser, Debug)]
pub struct CompactArg {
    #[arg(short, long, help = "difficulty (as floating point)")]
    pub diff: f64,
}

/// Validates that the input string is a valid 64-character hexadecimal string.
fn validate_hex_hash(hash: &str) -> Result<String, std::fmt::Error> {
    if hash.len() != 64 {
        return Err(std::fmt::Error);
    }

    if hex::decode(hash).is_err() {
        return Err(std::fmt::Error);
    }

    Ok(hash.to_string())
}

/// Custom parser to handle u128 with underscores
fn parse_u128_with_underscores(s: &str) -> Result<u128, std::num::ParseIntError> {
    let sanitized = s.replace('_', "");
    sanitized.parse::<u128>()
}
