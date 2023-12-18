use std::{fs, net::SocketAddr};

use clap::{App, Arg};
use serde_json::Value;

pub struct Config {
    pub data_bytes: usize,
    pub range_bits: usize,
    pub add_key_batch_size: usize,
    pub flp_batch_size: usize,
    pub unique_buckets: usize,
    pub threshold: f64,
    pub zipf_exponent: f64,
    pub server_0: SocketAddr,
    pub server_1: SocketAddr,
}

fn parse_ip(v: &Value, error_msg: &str) -> SocketAddr {
    v.as_str().expect(error_msg).parse().expect(error_msg)
}

pub fn get_config(filename: &str) -> Config {
    let json_data = &fs::read_to_string(filename).expect("Cannot open JSON file");
    let v: Value = serde_json::from_str(json_data).expect("Cannot parse JSON config");

    let range_bits: usize = v["range_bits"].as_u64().expect("Can't parse range_bits") as usize;
    let data_bytes: usize = v["data_bytes"].as_u64().expect("Can't parse data_bytes") as usize;
    let add_key_batch_size: usize = v["add_key_batch_size"]
        .as_u64()
        .expect("Can't parse add_key_batch_size") as usize;
    let flp_batch_size: usize = v["flp_batch_size"]
        .as_u64()
        .expect("Can't parse flp_batch_size") as usize;
    let unique_buckets: usize = v["unique_buckets"]
        .as_u64()
        .expect("Can't parse unique_buckets") as usize;
    let threshold = v["threshold"].as_f64().expect("Can't parse threshold");
    let zipf_exponent = v["zipf_exponent"]
        .as_f64()
        .expect("Can't parse zipf_exponent");
    let server_0 = parse_ip(&v["server_0"], "Can't parse server0 addr");
    let server_1 = parse_ip(&v["server_1"], "Can't parse server 1 addr");

    Config {
        data_bytes,
        range_bits,
        add_key_batch_size,
        flp_batch_size,
        unique_buckets,
        threshold,
        zipf_exponent,
        server_0,
        server_1,
    }
}

pub fn get_args(
    name: &str,
    get_server_id: bool,
    get_n_reqs: bool,
    get_malicious: bool,
) -> (Config, i8, usize, f32) {
    let mut flags = App::new(name)
        .version("0.1")
        .about("Mastic: Private Aggregated Statistics through Fully Linear Proofs")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILENAME")
                .help("Location of JSON config file")
                .required(true)
                .takes_value(true),
        );
    if get_server_id {
        flags = flags.arg(
            Arg::with_name("server_id")
                .short("i")
                .long("server_id")
                .value_name("NUMBER")
                .help("Zero-indexed ID of server")
                .required(true)
                .takes_value(true),
        );
    }
    if get_n_reqs {
        flags = flags.arg(
            Arg::with_name("num_clients")
                .short("n")
                .long("num_clients")
                .value_name("NUMBER")
                .help("Number of client requests to generate")
                .required(true)
                .takes_value(true),
        );
    }
    if get_malicious {
        flags = flags.arg(
            Arg::with_name("malicious")
                .short("m")
                .long("malicious")
                .value_name("NUMBER")
                .help("Percentage of malicious clients")
                .required(false)
                .takes_value(true),
        );
    }

    let flags = flags.get_matches();

    let mut server_id = -1;
    if get_server_id {
        server_id = flags.value_of("server_id").unwrap().parse().unwrap();
    }

    let mut n_reqs = 0;
    if get_n_reqs {
        n_reqs = flags.value_of("num_clients").unwrap().parse().unwrap();
    }

    let mut malicious = 0.0;
    if flags.is_present("malicious") {
        malicious = flags.value_of("malicious").unwrap().parse::<f32>().unwrap();
    }

    (
        get_config(flags.value_of("config").unwrap()),
        server_id,
        n_reqs,
        malicious,
    )
}
