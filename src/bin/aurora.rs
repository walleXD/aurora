use aurora::Cli;
use clap::Parser;
use human_panic::setup_panic;

fn main() {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    });

    let app = Cli::parse();
    app.exec().map_err(|e| panic!("{}", e)).unwrap();
}
