#![allow(unused)]

use clap::{Arg, ArgAction, Command, Parser, Subcommand};
use extract::*;
use serde::Serialize;
use serde_json::json;
use shared::Color;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod extract;
mod shared;

#[derive(Serialize, Default)]
struct Data {
    rgb_colors: Vec<[u8; 3]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    debug_message: Option<String>,
}

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

struct Debug {
    timer: Instant,
}

fn main() {
    let cli = Cli::parse();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let mut data = Data::default();

    let mut debug = cli.debug.then_some(Debug {
        timer: Instant::now(),
    });

    loop {
        let screenshot = get_image();
        if let Some(ref mut v) = debug {
            v.timer = Instant::now();
        }

        let samples = sample(&screenshot, cli.samples);
        let colors = mean(&samples);

        data.rgb_colors = colors.into_iter().map(|v| [v.r, v.g, v.b]).collect();
        if let Some(ref v) = debug {
            data.debug_message = Some(format!("algorithm: {:?}ms", v.timer.elapsed().as_millis()));
        }

        let json = serde_json::to_string(&data).unwrap();

        socket
            .send_to(
                json.as_bytes(),
                SocketAddr::new(cli.address.into(), cli.port),
            )
            .unwrap();

        sleep(Duration::from_millis(cli.period));
    }
}
