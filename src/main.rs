use fa_cose::collect;
use fa_cose::config::parse_config;
use fa_cose::file::FileHandler;
use fa_cose::terminal::Terminal;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    config_file: String,
}

fn do_main(args: Cli) {
    println!("=== FA COSE ===");

    if !args.config_file.is_empty() {
        let sensors = parse_config(&args.config_file);
        if sensors.is_ok() {
            for file in sensors.unwrap().files {
                collect(&file.get_abs_path(), FileHandler, Terminal);
            }
        }
    }
}

fn main() {
    let args = Cli::from_args();
    do_main(args);
}
