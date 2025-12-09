use std::collections::HashMap;

use rand::Rng;

#[derive(Debug)]
pub struct Clown {
    pub size: usize,
    pub body_parts: HashMap<usize, usize>
}

impl Clown {
    pub fn display(&self) {
        // head of the clown part
        println!(" _____");
        // for each body parts
        for i in 1..=self.body_parts.len() {
            // if the body part exist, we get its size
            if let Some(body_part_size) = self.body_parts.get(&i) {
                // size of the body part
                for _ in 1..=*body_part_size {
                    println!("|     |");
                }
            }
        }
        // footed of the clown part
        println!(" _____");
    }   
}

pub struct Game {
    pub nb_players: usize,
    pub current_turn: usize,
}

impl Game {
    pub fn roll(&self) -> usize {
        return rand::rng().random_range(1..=6);
    }
}
