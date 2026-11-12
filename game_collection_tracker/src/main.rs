#![allow(dead_code)]

mod rating;
mod game;
mod game_collection;
mod game_collection_tracker;

use game_collection_tracker::GameCollectionTracker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker: GameCollectionTracker = GameCollectionTracker::new();

    tracker.add_game()?;
    
    tracker.add_game()?;

    tracker.view_all_games();

    Ok(())
}
