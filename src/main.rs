use structopt::StructOpt;
use fa_cose::collect;
use fa_cose::terminal::Terminal;
use fa_cose::file::FileHandler;

#[derive(Debug, StructOpt)]
struct Cli {
    file: String,
}

fn do_main(args: Cli) {
    println!("=== FA COSE ===");

    if !args.file.is_empty() {
        collect(&args.file, FileHandler, Terminal);
    }
}

fn main() {
    let args = Cli::from_args();
    do_main(args);
}
