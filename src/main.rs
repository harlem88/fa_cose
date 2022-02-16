use structopt::StructOpt;

use fa_cose::config::parse_config;
use fa_cose::file::FileHandler;
use fa_cose::terminal::Terminal;
use fa_cose::{collect, schedule_telemetry};

#[derive(Debug, StructOpt)]
struct Cli {
    config_file: String,
}

#[tokio::main]
async fn main() {
    println!("=== FA COSE ===");

    let args = Cli::from_args();
    if !args.config_file.is_empty() {
        let sensors = parse_config(&args.config_file);
        if sensors.is_ok() {
            for file in sensors.unwrap().files {
                let id = file.get_abs_path();
                schedule_telemetry(id, &FileHandler, &Terminal, file.get_interval());
            }
        }
    }
    loop {}
}
