mod config;
mod stash;

fn main() {
    let configuration =  match config::load_configuration("config.json") {
        Err(e) => panic!(e),
        Ok(config) => config
    };

    println!("Got account: {}", configuration.account_name);

    // Load up the stash
    let contents = stash::load_currency_stash(&configuration, 1);

    // Load up conversion ratios

    // Append csv
}
