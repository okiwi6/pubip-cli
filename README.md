# PUBIP-CLI tool

The pubip cli tool is a very simple program to get the external IPv4 and save it to the clipboard.

## How to use PUBIP-CLI?

Run `pubip` and your ipv4 will magically appear in your clipboard

## Dependencies

pubip uses

- `reqwest`
- `rust-clipboard`
- external service: [icanhazip.com](https://icanhazip.com)

## Installing (Linux)

1.  `$ cd /opt`
2.  `$ git clone https://github.com/okiwi6/pubip-cli.git`
3.  `$ cd pubip-cli`
4.  `$ cargo build --release`
5.  `$ sudo ln -s /opt/pubip-cli/target/release/pubip /usr/local/bin/`
