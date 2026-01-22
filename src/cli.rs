use clap::{Parser, ValueEnum};
use std::net::SocketAddr;

#[derive(ValueEnum, Clone, Debug)]
pub enum TestMode {
    SameDomain,
    RandomSubdomain,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 't',
        long = "target",
        default_value = "192.168.2.1:53",
        help = "DNS server target address (host:port)"
    )]
    pub target: SocketAddr,

    #[arg(
        short = 'd',
        long = "domain",
        default_value = "example.com",
        help = "Domain name to query"
    )]
    pub domain: String,

    #[arg(
        short = 'c',
        long = "count",
        default_value_t = 500000,
        help = "Total number of queries to send"
    )]
    pub count: u64,

    #[arg(
        short = 'o',
        long = "timeout",
        default_value_t = 5,
        help = "Timeout in seconds for each query"
    )]
    pub timeout: u64,

    #[arg(
        short = 'w',
        long = "workers",
        default_value_t = 10,
        help = "Number of concurrent workers"
    )]
    pub workers: usize,

    #[arg(
        short = 'm',
        long = "mode",
        value_enum,
        default_value_t = TestMode::SameDomain,
        help = "Test mode: same-domain or random-subdomain"
    )]
    pub mode: TestMode,
}

impl Cli {
    pub fn parse_args() -> Self {
        let args = Self::parse();

        println!("DNS Benchmark Tool");
        println!("Target: {}", args.target);
        println!("Domain: {}", args.domain);
        println!("Total queries: {}", args.count);
        println!("Timeout: {} seconds", args.timeout);
        println!("Workers: {}", args.workers);
        println!("Test mode: {:?}", args.mode);
        println!("----------------------------------------");

        args
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.count == 0 {
            return Err("Count must be greater than 0".to_string());
        }

        if self.timeout == 0 {
            return Err("Timeout must be greater than 0".to_string());
        }

        Ok(())
    }
}
