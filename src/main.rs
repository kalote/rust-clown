use std::{collections::HashMap, io};

use crate::types::{Clown, Game};
mod types;

// number of turns / max sixe of the clown
const MAX_TURN: usize = 4;

fn main() {
    let mut nb_players = String::new();
    // hashmap to store clown for each players
    let mut scores: HashMap<usize, Clown> = HashMap::new();

    println!("Clown game!");
    println!("-----------");
    println!("Please enter nb of players:");

    io::stdin()
        .read_line(&mut nb_players)
        .expect("Issue while ingesting data");
    
    let nb_players:usize = match nb_players.trim()
                                .parse() {
                                    Ok(n) if n > 0 => n,
                                    _ => {
                                        println!("invalid value. Will use 1.");
                                        1
                                    }
                                };

    // start a game
    let mut new_game= Game {
        current_turn: 1,
        nb_players
    };
    
    loop {
        for i in 1..=new_game.nb_players {
            // for each player, we get its entry in the hashMap or we create one
            let player_clown = scores.entry(i)
                .or_insert(Clown {
                    size: 0,
                    body_parts: HashMap::new()
                });
            println!("Player {}'s turn (turn {}/{})", i, new_game.current_turn, MAX_TURN);
            
            let score = new_game.roll();
            println!("You scored {}!", score);
            
            player_clown.size += score;
            player_clown.body_parts
                .insert(new_game.current_turn, score);
            player_clown.display();
        }
        if new_game.current_turn >= MAX_TURN {
            println!("Game is done! Here are the final scores (highest is the winner):");
            for (id, player_clown) in &scores {
                println!("Player {}: {} points", id, player_clown.size);
            }
            break;
        }
        new_game.current_turn += 1 ;
    }
}
