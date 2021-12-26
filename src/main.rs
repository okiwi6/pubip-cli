extern crate clipboard;
extern crate reqwest;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn make_ipv4_request() -> String {
    let mut resp = reqwest::blocking::get("https://ipv4.icanhazip.com").unwrap();
    let mut buffer: Vec<u8> = vec![];
    if resp.status() == 200 {
        resp.copy_to(&mut buffer).expect("Could not read body");
        let body_as_str = std::str::from_utf8(&buffer).expect("Could not decode body");
        return body_as_str[..body_as_str.len() - 1].to_owned();
    }
    panic!("Received {} from server", resp.status());
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(make_ipv4_request()).unwrap();
    println!("{:?} set to clipboard", ctx.get_contents().unwrap());
}
