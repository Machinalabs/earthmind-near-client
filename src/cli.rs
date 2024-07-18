use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "earthmind_client_near")]
#[command(about = "A Near block listener with different modes", long_about = None)]
pub struct Cli {
    // to choose if is a miner o validator
    #[arg(long, value_enum, default_value_t = Modes::Miner)]
    pub mode: Modes,

    #[arg(long)]
    pub private_key: String,

    #[arg(long, default_value = "testnet")]
    pub network: Networks,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Modes {
    Miner,
    Validator,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Networks {
    Testnet,
    Mainnet,
}
