# crypto-password

Generates a random, memorable passphrase: a 12-word [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonic joined with hyphens.

```console
$ crypto-password
warrior-faculty-add-force-life-ready-express-tourist-poet-govern-reopen-permit
```

## How it works

1. Reads 16 bytes (128 bits) of entropy from the operating system's CSPRNG via [`getrandom`](https://crates.io/crates/getrandom).
2. Encodes them as a 12-word English BIP39 mnemonic (including the 4-bit checksum) with [`bip39`](https://crates.io/crates/bip39) from the rust-bitcoin project.
3. Prints the words separated by `-`.

The entropy, mnemonic and output string are wiped from memory on drop using [`zeroize`](https://crates.io/crates/zeroize).

Each passphrase carries 128 bits of entropy, the same strength as a 12-word wallet seed.

## Install

Requires Rust 1.85 or newer.

```sh
cargo install --git https://github.com/peterpeerdeman/crypto-password
```

Or from a local checkout:

```sh
cargo install --path .
```

## Development

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo audit
```

## Note

The output is a valid BIP39 mnemonic. Using it as a password is fine; if you use one as a cryptocurrency wallet seed, generate it on a trusted, offline machine.

## License

[MIT](LICENSE)
