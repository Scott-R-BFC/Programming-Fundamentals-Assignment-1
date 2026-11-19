//! # Game Collection Tracker
//!
//! The program instantiates a `GameCollectionTracker` and calls its `menu_loop` method.
//! This method allows the user to perform the operations defined in the high level requirements
//! section of the accompanying assignment document.
//!
//! It does so whilst making use of structured and object-oriented
//! programming principles; the use of procedures, classes and reusable components; appropriate
//! input validation and error handling; logical, maintainable and scalable code and meaningful
//! internal code comments.
//!
//! Below is the documentation for the modules used to build the program.
//!
//! Each `module` contains relevant constant definitions, a `struct` whose name aligns with the
//! module and an `impl` block which provides implementation of the `struct`'s methods.
//!
//! The `GameCollectionTracker` module also contains two tables describing data and functions
//! associated with:
//! - operations the user can select from the menu
//! - attributes associated with a `Game`
//! - retrieving `Game`s from a `GameCollection`

#![allow(dead_code)]

mod game;
mod game_collection;
mod game_collection_tracker;
mod rating;

use game_collection_tracker::GameCollectionTracker;

/// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    GameCollectionTracker::new().menu_loop()
}
