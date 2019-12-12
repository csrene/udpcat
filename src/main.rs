use std::net::UdpSocket;

use std::path::PathBuf;
use structopt::StructOpt;
use std::time::Instant;

/// UDPcat
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opts {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Port to listen
    #[structopt(short, long)]
    port: u16,

    /// Host to listen
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v: print the source ip. -vvv: print the timestamp and the source ip)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Max message length in bytes
    #[structopt(short, long, default_value = "1000")]
    max_length: usize,
//
//    /// Render newline inside messages
//    #[structopt(short, long)]
//    render_newline: bool
}

fn main() -> std::io::Result<()> {use std::time::{SystemTime, UNIX_EPOCH};



    let opts: Opts = Opts::from_args();
    dbg!(&opts.host);
    let socket =  UdpSocket::bind(format!("{}:{}", opts.host, opts.port))?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = vec![0; opts.max_length];

    loop {

        let (amt, src) = socket.recv_from(&mut buf)?;

        let res = std::str::from_utf8(&buf[0..amt]).unwrap_or("eerrroooor");

        match opts.verbose {
            0 => println!("{}", res),
            1 => println!("source: {}; {}", &src, res),
            3 => {
                let now =  SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH).expect("se fodeu")
                    .as_millis();
                println!("timestamp: {}; source: {}; {}", now, &src, res)
            },
            _ => println!("{}", res),
        }
    }

    Ok(())
}