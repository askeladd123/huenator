#![allow(unused)]

use clap::{Arg, ArgAction, Command, Parser, Subcommand};
use extract::*;
use serde_json::json;
use shared::Color;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

mod extract;
mod gui;
mod shared;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST)]
    address: Ipv4Addr,

    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    #[arg(short = 't', long, default_value_t = 1000)]
    period: u64,

    #[arg(short, long, default_value_t = 1000)]
    samples: u64,

    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    loop {
        let screenshot = get_image();
        let samples = sample(&screenshot, 1000);
        let colors = mean(&samples);
        let colors = colors
            .into_iter()
            .map(|v| [v.r, v.g, v.b])
            .collect::<Vec<_>>();
        let json = serde_json::to_string(&colors).unwrap();

        socket
            .send_to(
                json.as_bytes(),
                SocketAddr::new(cli.address.into(), cli.port),
            )
            .unwrap();

        sleep(Duration::from_millis(cli.period));
    }
}
