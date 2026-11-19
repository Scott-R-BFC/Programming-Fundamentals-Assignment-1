//! # Game Collection Tracker
//!
//! #### Program Info
//!
//! The program entry point is [main] which instantiates a [game_collection_tracker::GameCollectionTracker] and calls its [game_collection_tracker::GameCollectionTracker::menu_loop] method.
//! This method allows the user to perform the operations defined in the high level requirements
//! section of the accompanying assignment document.
//!
//! It does so whilst making use of structured and object-oriented
//! programming principles; the use of procedures, classes and reusable components; appropriate
//! input validation and error handling; logical, maintainable and scalable code and meaningful
//! internal code comments.
//!
//! When the [game_collection_tracker::GameCollectionTracker::menu_loop] has been executed, the
//! program terminates
//!
//!
//! #### Module Info
//!
//! Each module contains relevant constant definitions, a `struct` whose name aligns with the
//! module and an `impl` block which provides implementation of the `struct`'s methods.
//!
//! The [game_collection_tracker] module also contains two tables describing data and functions
//! associated with:
//! - operations the user can select from the menu
//! - attributes associated with a [game::Game]
//! - retrieving [game::Game]s from a [game_collection::GameCollection]

#![allow(dead_code)]

mod game;
mod game_collection;
mod game_collection_tracker;
mod rating;

use game_collection_tracker::GameCollectionTracker;

/// Program entry point: Executes IO loop as described in [Game Collection Tracker](##irogram-info).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    GameCollectionTracker::new().menu_loop()
}
