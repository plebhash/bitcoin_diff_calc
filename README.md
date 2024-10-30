<h1 align="center">
⛏ bitcoin_diff_calc 🎯
<br>
</h1>

<h3 align="center">
a convenient toolkit for calculations on Bitcoin mining difficulty
</h3>

## features

- **Library Functions**: Reusable functions for calculating:
    - Mining difficulty based on hashrate and block time.
    - Mining difficulty of some specific hash.
    - Target value based on difficulty.
    - Compact target representation.
- **Command-Line Interface (CLI)**:
    - `difficulty` subcommand: Calculate mining difficulty based on hashrate and block time.
    - `difficulty-from-hash` subcommand: Calculate mining difficulty of some specificy hash.
    - `compact` subcommand: Calculate the compact target for a given difficulty.

## CLI

```shell
$ ./target/release/bitcoin_diff_calc -h
a convenient toolkit for calculations on Bitcoin mining difficulty

Usage: bitcoin_diff_calc <COMMAND>

Commands:
  difficulty  Calculate difficulty based on hashrate and average block time
  difficulty-from-hash  Calculate difficulty based on some specific hash
  compact     Calculate compact target for some difficulty (as floating point)
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `difficulty` subcommand

This command is useful for estimating the difficulty based on:
- hashrate (in H/s)
- how often you expect the blocks (or shares) to be found (on average)

```shell
$ ./target/release/bitcoin_diff_calc difficulty -h
Calculate difficulty based on hashrate and average block time

Usage: bitcoin_diff_calc difficulty --hashrate <HASHRATE> --block-time <BLOCK_TIME>

Options:
  -r, --hashrate <HASHRATE>      network hashrate (H/s)
  -b, --block-time <BLOCK_TIME>  average block time (s)
  -h, --help                     Print help
```

For example, imagine:
- hashrate: 100 TH/s (100_000_000_000_000 H/s)
- average block time: 5 minutes (5 * 60 = 300 s)

```shell
$ ./target/release/bitcoin_diff_calc difficulty -r 100_000_000_000_000 -b 300
difficulty: 6984919
```

### `difficulty-from-hash` subcommand

This command is useful for calculating the difficulty of some specific hash:

```shell
$ ./target/release/bitcoin_diff_calc difficulty-from-hash -h
Calculate difficulty based on some specific hash

Usage: bitcoin_diff_calc difficulty-from-hash --hash <HASH>

Options:
  -s, --hash <HASH>  hash
  -h, --help         Print help
```

For example:
```shell
$ ./target/release/bitcoin_diff_calc difficulty-from-hash -s 000000000008e36f853a1a994299a21faf7b8f1465ed7f33f732c479f4780a08
difficulty: 7373.076084262154
```

### `compact` subcommand

Bitcoin block headers carry the current target in the `bits` (a.k.a `nBits`) field [under compact representation](https://learnmeabitcoin.com/technical/block/bits/).

This command is useful for finding the compact representation of some specific target by providing the corresponding difficulty.

```shell
$ ./target/release/bitcoin_diff_calc compact -h
Calculate compact target for some difficulty (as floating point)

Usage: bitcoin_diff_calc compact --diff <DIFF>

Options:
  -d, --diff <DIFF>  difficulty (as floating point)
  -h, --help         Print help
```

For example:
```shell
$ ./target/release/bitcoin_diff_calc compact -d 95672703408223.94
compact target: 0x1702f128
```

## license

GNU General Public License v3.0