use clap::Parser;

mod cli;
use cli::*;

fn main() {
    let cli = crate::cli::Cli::parse();

    match &cli.command {
        Commands::Difficulty(args) => {
            let hashrate = args.hashrate;
            let block_time = args.block_time;
            let diff =
                bitcoin_diff_calc::difficulty_from_hashrate_and_block_time(hashrate, block_time);
            println!("difficulty: {}", diff);
        }
        Commands::DifficultyFromHash(args) => {
            let hash = args.hash.clone();
            let diff = bitcoin_diff_calc::difficulty_from_hash(&hash);
            println!("difficulty: {}", diff);
        }
        Commands::Compact(args) => {
            let diff = args.diff;
            let compact_target = bitcoin_diff_calc::difficulty_to_compact(diff);
            println!("compact target: {:#010x}", compact_target);
        }
    };
}
