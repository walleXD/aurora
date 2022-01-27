use aurora::Cli;
use clap::Parser;

fn main() {
    let app = Cli::parse();
    app.exec()
}
