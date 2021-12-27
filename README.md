# PUBIP-CLI tool

If you use minecrafts "open to lan" feature, this tool automatically determines your
public IP address as well as the port and lays it into the system clipboard so that you can easily share the address with your friends.

## How to use PUBIP-CLI?

Run `pubip` and your ipv4 will magically appear in your clipboard

## Dependencies

pubip uses

- `reqwest`
- `rust-clipboard`
- external service: [icanhazip.com](https://icanhazip.com)

## Installing (Linux)

1.  Choose a location, for example

    `$ cd /opt`

2.  Clone the repo

    `$ git clone https://github.com/okiwi6/pubip-cli.git`

3.  Enter the repo

    `$ cd pubip-cli`

4.  Build the tool using cargo [(if you don't have the rust toolchain installed, do so first)](https://www.rust-lang.org/tools/install)

    `$ cargo build --release`

5.  If you want to, create a symlink so you can use the tool from the command line

    `$ sudo ln -s /opt/pubip-cli/target/release/pubip /usr/local/bin/`
