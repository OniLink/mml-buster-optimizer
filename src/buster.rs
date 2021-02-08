use std::cmp::min;
use std::vec::Vec;
use crate::config::Config;


#[derive(Clone)]
pub struct BusterPart {
    pub name: String,  // Name of the Buster part
    pub attack: u8,    // Attack stat
    pub energy: u8,    // Energy stat
    pub range: u8,     // Range stat
    pub rapid: u8,     // Rapid stat
}


#[derive(Clone)]
pub struct BusterLayout {
    pub buster_parts: Vec< BusterPart >  // The buster parts being used
}


impl BusterLayout {
    pub fn empty() -> BusterLayout {
        BusterLayout { buster_parts: Vec::new() }
    }


    pub fn clamp_stats( config: &Config, attack: u8, energy: u8, range: u8, rapid: u8 ) -> ( u8, u8, u8, u8 ) {
        // Max values
        let attack_max = 7;
        let energy_max = 7;
        let range_max = 7;
        let rapid_max = match config.sequel {
            false => 4,
            true => 7
        };

        // Clamp
        let attack_val = min( attack, attack_max );
        let energy_val = min( energy, energy_max );
        let range_val = min( range, range_max );
        let rapid_val = min( rapid, rapid_max );

        ( attack_val, energy_val, range_val, rapid_val )
    }


    pub fn score_stats( config: &Config, attack: u8, energy: u8, range: u8, rapid: u8 ) -> i32 {
        // Clamp the stats
        let ( attack_val, energy_val, range_val, rapid_val )
            = BusterLayout::clamp_stats( config, attack, energy, range, rapid );
        let attack = attack_val as i32;
        let energy = energy_val as i32;
        let range = range_val as i32;
        let rapid = rapid_val as i32;
    
        // Score the layout
        let score = config.attack_weight * attack
            + config.energy_weight * energy
            + config.range_weight * range
            + config.rapid_weight * rapid;

        score
    }

/*
    pub fn score_stats_tup( config: &Config, stats: ( u8, u8, u8, u8 ) ) -> i32 {
        let ( attack, energy, range, rapid ) = stats;
        BusterLayout::score_stats( config, attack, energy, range, rapid )
    }
*/

    pub fn stats( &self, config: &Config ) -> ( u8, u8, u8, u8 ) {
        // Tally the stats
        let mut attack_val = 1;
        let mut energy_val = 1;
        let mut range_val = 1;
        let mut rapid_val = 1;

        for part in &self.buster_parts {
            attack_val += part.attack;
            energy_val += part.energy;
            range_val += part.range;
            rapid_val += part.rapid;
        }

        BusterLayout::clamp_stats( config, attack_val, energy_val, range_val, rapid_val )
    }


    pub fn score( &self, config: &Config ) -> i32 {
        let ( attack_val, energy_val, range_val, rapid_val ) = self.stats( config );

        BusterLayout::score_stats( config, attack_val, energy_val, range_val, rapid_val )
    }


    pub fn print_stats( &self, config: &Config ) {
        let ( attack_stat, energy_stat, range_stat, rapid_stat ) = self.stats( config );

        println!( "Score {}", self.score( config ) );
        println!( "Parts:" );
        for part in &self.buster_parts {
            println!( "  {}", part.name );
        }

        println!( "Buster Stats: Attack = {}, Energy = {}, Range = {}, Rapid = {}",
                  attack_stat, energy_stat, range_stat, rapid_stat );
    }
}

