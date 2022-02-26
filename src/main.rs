use structopt::StructOpt;

use fa_cose::astarte::{init_astarte, Astarte};
use fa_cose::config::parse_config;
use fa_cose::file::FileHandler;
use fa_cose::schedule_telemetry;

#[derive(Debug, StructOpt)]
struct Cli {
    config_file: String,
}

#[tokio::main]
async fn main() {
    println!("=== FA COSE ===");

    let args = Cli::from_args();
    if args.config_file.is_empty() {
        return;
    }

    let config = parse_config(&args.config_file);
    if config.is_err() {
        println!("Unable to parse config: {}", config.err().unwrap());
        return;
    }

    let config = config.unwrap();
    let sdk_result = init_astarte(config.astarte_device_params).await;
    if sdk_result.is_err() {
        println!("Unable to init astarte sdk {:?}", sdk_result.err().unwrap());
        return;
    }

    let mut astarte = Astarte {
        device_sdk: sdk_result.unwrap(),
    };

    for file in config.sensor.files {
        let topic = file.get_topic();
        let astarte = astarte.clone();
        schedule_telemetry(topic, &FileHandler, astarte, file.get_interval());
    }

    loop {
        match astarte.device_sdk.poll().await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
