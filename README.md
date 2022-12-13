# solfmt

Formats output of Solana's `cargo test-bpf/test-sbf` command.

## Installation

```sh
cargo install solfmt
```

## Usage

Run the your test command as usual, pipe `stderr` into `stdout` and pipe the result to
`solfmt`.

```sh
cargo test-sbf -- --test-threads=1 2>&1 | solfmt
```

## LICENSE

MIT
