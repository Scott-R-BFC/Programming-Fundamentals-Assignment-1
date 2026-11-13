#![allow(dead_code)]

mod game;
mod game_collection;
mod game_collection_tracker;
mod rating;

use game_collection_tracker::GameCollectionTracker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker: GameCollectionTracker = GameCollectionTracker::new();

    tracker.menu_loop()?;

    Ok(())
}
