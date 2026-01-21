mod dns;
mod benchmark;
mod cli;

use cli::Cli;
use dns::DnsBenchmark as DnsBenchmarkStruct;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse_args();
    
    if let Err(err) = args.validate() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    
    let benchmark = DnsBenchmarkStruct::new(
        args.target,
        args.domain,
        args.rate,
        Duration::from_secs(args.timeout),
        args.mode,
    );
    
    benchmark.run(args.count).await?;
    
    Ok(())
}
