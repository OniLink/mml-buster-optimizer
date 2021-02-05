mod buster;
mod config;
mod optimizer;

//use crate::buster::BusterLayout;
use crate::config::Config;
use crate::optimizer::simulate;


fn main() {
    // Load the configuration
    let config = Config::new();

    // Simulate to find the best layout
    let best_layout = simulate( &config );

    // Show results
    best_layout.print_stats( &config );
}
