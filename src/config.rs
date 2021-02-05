extern crate clap;

use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::vec::Vec;
use clap::{ Arg, App, crate_version };
use crate::buster::BusterPart;


pub struct Config {
    pub adapter_plug: bool,              // Does the player have the Adapter Plug (3 buster parts instead of 2)
    pub attack_weight: i32,              // Weight for the Attack stat
    pub energy_weight: i32,              // Weight for the Energy stat
    pub range_weight: i32,               // Weight for the Range stat
    pub rapid_weight: i32,               // Weight for the Rapid stat
    pub sequel: bool,                    // Is the player playing Legends 2 (Rapid limit is 7 instead of 4)
    pub buster_parts: Vec< BusterPart >  // The player's available Buster parts
}


impl Config {
    pub fn new() -> Config {
        // Use clap to get the command line arguments
        let config_matches = App::new( "Mega Man Legends Buster Optimizer" )
            .version( crate_version!() )
            .author( "Violet Kurtz <vi.kurtz@protonmail.com>" )
            .about( "Finds optimal configurations for your Buster parts" )
            .arg( Arg::with_name( "BUSTER_FILE" )
                  .help( "JSON file containing available buster parts" )
                  .required( true )
                  .index( 1 ) )
            .arg( Arg::with_name( "ADAPTER_PLUG" )
                  .help( "Set to indicate you have the Adapter Plug" )
                  .short( "a" )
                  .long( "adapter" ) )
            .arg( Arg::with_name( "SEQUEL" )
                  .help( "Set to indicate you are playing Legends 2" )
                  .short( "s" )
                  .long( "sequel" ) )
            .arg( Arg::with_name( "ATTACK_WEIGHT" )
                  .help( "Weight for the Attack stat" )
                  .long( "attack" )
                  .takes_value( true )
                  .default_value( "1" ) )
            .arg( Arg::with_name( "ENERGY_WEIGHT" )
                  .help( "Weight for the Energy stat" )
                  .long( "energy" )
                  .takes_value( true )
                  .default_value( "1" ) )
            .arg( Arg::with_name( "RANGE_WEIGHT" )
                  .help( "Weight for the Range stat" )
                  .long( "range" )
                  .takes_value( true )
                  .default_value( "1" ) )
            .arg( Arg::with_name( "RAPID_WEIGHT" )
                  .help( "Weight for the Rapid stat" )
                  .long( "rapid" )
                  .takes_value( true )
                  .default_value( "1" ) )
            .get_matches();

        // Parse options into appropriate types
        let buster_filename: String = config_matches.value_of( "BUSTER_FILE" )
            .unwrap().to_string();
        let adapter_plug: bool = config_matches.is_present( "ADAPTER_PLUG" );
        let sequel: bool = config_matches.is_present( "SEQUEL" );
        let attack_weight: i32 = config_matches.value_of( "ATTACK_WEIGHT" )
            .unwrap().parse().unwrap();
        let energy_weight: i32 = config_matches.value_of( "ENERGY_WEIGHT" )
            .unwrap().parse().unwrap();
        let range_weight: i32 = config_matches.value_of( "RANGE_WEIGHT" )
            .unwrap().parse().unwrap();
        let rapid_weight: i32 = config_matches.value_of( "RAPID_WEIGHT" )
            .unwrap().parse().unwrap();

        // Parse the Buster parts file
        let buster_parts = parse_buster_parts( buster_filename );

        // Return the config info
        Config{ adapter_plug, attack_weight, energy_weight, range_weight, rapid_weight, sequel, buster_parts }
    }
}


/*
 * Grammar for Buster Parts file:
 * name,atk,eng,rng,rpd
 *
 * atk, eng, rng, and rpd are all u8s
 * Each line is one buster part
 */
fn parse_buster_parts( filename: String ) -> Vec< BusterPart > {
    // Load the file
    let file = File::open( &filename ).unwrap();
    let reader = BufReader::new( file );

    let mut buster_parts: Vec< BusterPart > = Vec::new();

    // Read line-by-line
    for ( line_num, line_data ) in reader.lines().enumerate() {
        let line = line_data.unwrap();

        // Skip commented and blank lines
        if line.len() == 0 || line.starts_with( '#' ) || line.starts_with( "//" ) {
            continue;
        }

        // Comma separated!
        let tokens: Vec< &str > = line.split( ',' ).collect();

        // Error check: skip lines with =/= 5 entries
        if tokens.len() != 5 {
            println!( "Bad line #{} in {}", line_num, &filename );
            continue;
        }

        // Parse
        let name: String = tokens[ 0 ].to_string();
        let attack: u8 = tokens[ 1 ].parse().unwrap();
        let energy: u8 = tokens[ 2 ].parse().unwrap();
        let range: u8 = tokens[ 3 ].parse().unwrap();
        let rapid: u8 = tokens[ 4 ].parse().unwrap();

        buster_parts.push( BusterPart{ name, attack, energy, range, rapid } );
    }

    buster_parts
}

