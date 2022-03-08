use dds_core::server::init_and_run_server;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "DDS", about = "Decentralized Data Science")]
struct CommandLineArgs {
    /// Address of DDS server
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    address: String,

    /// Port of DDS server
    #[structopt(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    let CommandLineArgs { address, port } = CommandLineArgs::from_args();

    init_and_run_server(address, port).await;

    Ok(())
}
