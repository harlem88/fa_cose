use fa_cose::gpio::GPIO;
use fa_cose::terminal::Terminal;
use fa_cose::{collect, Publisher, Sensor, Switch};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    command: String,
}

fn do_main(args: Cli) {
    println!("=== FA COSE ===");

    if !args.command.is_empty() {
        collect(&args.command, Terminal, Terminal);
    }
}

fn main() {
    let args = Cli::from_args();
    do_main(args);
}


