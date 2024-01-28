use clap::Parser;
use log::LevelFilter;

use crate::Error;

#[derive(Parser, Debug)]
#[clap(name = "nexellia-miner", version, about = "A Nexellia high performance CPU miner", term_width = 0)]
pub struct Opt {
    #[clap(short, long, help = "Enable debug logging level")]
    pub debug: bool,
    #[clap(short = 'a', long = "mining-address", help = "The Nexellia address for the miner reward")]
    pub mining_address: String,
    #[clap(short = 's', long = "nexelliad-address", default_value = "127.0.0.1", help = "The IP of the nexelliad instance")]
    pub nexelliad_address: String,

    #[clap(long = "devfund-percent", help = "The percentage of blocks to send to the devfund (minimum 0%)", default_value = "0", parse(try_from_str = parse_devfund_percent))]
    pub devfund_percent: u16,

    #[clap(short, long, help = "nexelliad port [default: Mainnet = 33455, Testnet = 33555]")]
    port: Option<u16>,

    #[clap(long, help = "Use testnet instead of mainnet [default: false]")]
    testnet: bool,
    #[clap(short = 't', long = "threads", help = "Amount of CPU miner threads to launch [default: 0]")]
    pub num_threads: Option<u16>,
    #[clap(
        long = "mine-when-not-synced",
        help = "Mine even when nexelliad says it is not synced",
        long_help = "Mine even when nexelliad says it is not synced, only useful when passing `--allow-submit-block-when-not-synced` to nexelliad  [default: false]"
    )]
    pub mine_when_not_synced: bool,

    #[clap(skip)]
    pub devfund_address: String,
}

fn parse_devfund_percent(s: &str) -> Result<u16, &'static str> {
    let err = "devfund-percent should be --devfund-percent=XX.YY up to 2 numbers after the dot";
    let mut splited = s.split('.');
    let prefix = splited.next().ok_or(err)?;
    // if there's no postfix then it's 0.
    let postfix = splited.next().ok_or(err).unwrap_or("0");
    // error if there's more than a single dot
    if splited.next().is_some() {
        return Err(err);
    };
    // error if there are more than 2 numbers before or after the dot
    if prefix.len() > 2 || postfix.len() > 2 {
        return Err(err);
    }
    let postfix: u16 = postfix.parse().map_err(|_| err)?;
    let prefix: u16 = prefix.parse().map_err(|_| err)?;
    // can't be more than 99.99%,
    if prefix >= 100 || postfix >= 100 {
        return Err(err);
    }
    /*
    if prefix < 2 {
        // Force at least 2 percent
        return Ok(200u16);
    }
    */
    // DevFund is out of 10_000
    Ok(prefix * 100 + postfix)
}

impl Opt {
    pub fn process(&mut self) -> Result<(), Error> {
        //self.gpus = None;
        if self.nexelliad_address.is_empty() {
            self.nexelliad_address = "127.0.0.1".to_string();
        }

        if !self.nexelliad_address.contains("://") {
            let port_str = self.port().to_string();
            let (nexelliad, port) = match self.nexelliad_address.contains(':') {
                true => self.nexelliad_address.split_once(':').expect("We checked for `:`"),
                false => (self.nexelliad_address.as_str(), port_str.as_str()),
            };
            self.nexelliad_address = format!("grpc://{}:{}", nexelliad, port);
        }
        log::info!("nexelliad address: {}", self.nexelliad_address);

        if self.num_threads.is_none() {
            self.num_threads = Some(0);
        }

        let miner_network = self.mining_address.split(':').next();
        self.devfund_address = String::from("nexellia:qrfekdrsdxw3tn6qcxm2e53rm6873a8003uwk2a7tudwfp886q4xvc47fgvpn");
        let devfund_network = self.devfund_address.split(':').next();
        if miner_network.is_some() && devfund_network.is_some() && miner_network != devfund_network {
            self.devfund_percent = 0;
            log::info!(
                "Mining address ({}) and devfund ({}) are not from the same network. Disabling devfund.",
                miner_network.unwrap(),
                devfund_network.unwrap()
            )
        }
        Ok(())
    }

    fn port(&mut self) -> u16 {
        *self.port.get_or_insert(if self.testnet { 42211 } else { 33455 })
    }

    pub fn log_level(&self) -> LevelFilter {
        if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }
}
