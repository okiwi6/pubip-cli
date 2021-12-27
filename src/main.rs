extern crate clipboard;
extern crate reqwest;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::process::Command;

type Port = u32;

fn get_open_tcp_ports() -> Vec<Port> {
    let output = Command::new("netstat")
        .arg("-tnlp")
        .output()
        .expect("failed to execute process");
    let output_str = std::str::from_utf8(&output.stdout).expect("Can't list ports");
    let mut candidates = vec![];

    for line in output_str.split("\n") {
        if line.contains("java") {
            let port = line
                .split(" ")
                .filter(|l| l.len() > 0)
                .nth(3)
                .unwrap()
                .split(":")
                .last()
                .unwrap()
                .parse::<Port>()
                .unwrap();
            candidates.push(port);
        }
    }
    return candidates;
}

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

    let external_ipv4 = make_ipv4_request();
    let ports = get_open_tcp_ports();

    if ports.len() != 1 {
        eprintln!("Port can not be determined");
        if ports.len() > 0 {
            eprintln!("However it must be one of {:?}", ports);
        }
        ctx.set_contents(external_ipv4).unwrap();
    } else {
        ctx.set_contents(format!("{}:{}", external_ipv4, ports[0]))
            .unwrap();
    }
    println!("{:?} set to clipboard", ctx.get_contents().unwrap());
}
