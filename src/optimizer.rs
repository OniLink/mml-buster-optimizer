use std::vec::Vec;
use crate::buster::BusterLayout;
use crate::config::Config;


pub fn simulate( config: &Config ) -> BusterLayout {
    let part_count = match config.adapter_plug {
        false => 2,
        true => 3
    };

    let available_parts = config.buster_parts.len();

    // Treat this as the Knapsack problem
    // Create the table of scores for each stat
    let mut table: Vec< Vec< ( u8, u8, u8, u8 ) > > = Vec::with_capacity( part_count + 1 );

    for _i in 0..=available_parts {
        let mut column: Vec< ( u8, u8, u8, u8 ) > = Vec::with_capacity( available_parts + 1 );

        for _j in 0..=part_count {
            column.push( ( 1u8, 1u8, 1u8, 1u8 ) );
        }
        
        table.push( column );
    }
    
    // Iterate the buster parts
    for i in 0..available_parts {
        let col = i+1;  // Indexing is annoying, the 0th column is the "no parts" column

        // Pull out the current part's stats now so we only do it once
        let current_attack = config.buster_parts[ i ].attack;
        let current_energy = config.buster_parts[ i ].energy;
        let current_range = config.buster_parts[ i ].range;
        let current_rapid = config.buster_parts[ i ].rapid;

        // Fill the table with the buster part
        for j in 0..part_count {
            let row = j+1;  // Indexing is annoying, the 0th row is the "unequipped" column

            // Find the score of not taking the part
            let ( attack_no, energy_no, range_no, rapid_no ) = table[ col-1 ][ row ];
            let score_no = BusterLayout::score_stats( config, attack_no, energy_no, range_no, rapid_no );

            // Find the score of taking the part
            let ( attack_yes_part, energy_yes_part, range_yes_part, rapid_yes_part ) = table[ col-1 ][ row-1 ];
            let attack_yes = attack_yes_part + current_attack;
            let energy_yes = energy_yes_part + current_energy;
            let range_yes = range_yes_part + current_range;
            let rapid_yes = rapid_yes_part + current_rapid;
            let score_yes = BusterLayout::score_stats( config, attack_yes, energy_yes, range_yes, rapid_yes );

            // Pick the best option
            if score_yes > score_no {
                table[ col ][ row ] = ( attack_yes, energy_yes, range_yes, rapid_yes );
            } else {
                table[ col ][ row ] = ( attack_no, energy_no, range_no, rapid_no );
            }
        }
    }

    // With the table built, we can run back to build our optimal Buster Layout
    let mut layout = BusterLayout::empty();

    // Start from the bottom right ("optimal") solution
    let mut col = available_parts;
    let mut row = part_count;

    // Work towards the top left
    while col > 0 && row > 0 {
        // Move up the table as far as we can
        while BusterLayout::score_stats_tup( config, table[ col ][ row ] ) == BusterLayout::score_stats_tup( config, table[ col ][ row-1 ] ) {
            row -= 1;
        }

        // Move left the table as far as we can
        while BusterLayout::score_stats_tup( config, table[ col ][ row ] ) == BusterLayout::score_stats_tup( config, table[ col-1 ][ row ] ) {
            col -= 1;
        }

        // When we're at the top-left of our set, we found the next item to add
        layout.buster_parts.push( config.buster_parts[ col-1 ].clone() );
        col -= 1;
        row -= 1;
    }

    layout.buster_parts.reverse();

    layout
}
