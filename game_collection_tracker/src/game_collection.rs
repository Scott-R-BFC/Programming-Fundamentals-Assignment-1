use crate::game::Game;
use crate::rating::Rating;

use std::error::Error;

pub struct GameCollection {
    name: Option<String>,
    owner: Option<String>,
    games: Vec<Game>,
}

impl GameCollection {
    // constructor
    pub fn new() -> GameCollection {
        GameCollection {
            name: None,
            owner: None,
            games: Vec::new(),
        }
    }

    // getters
    pub fn get_games_with_title(&self, title: &str) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| game.get_title() == Some(title))
            .collect()
    }
    pub fn get_games_with_genre(&self, genre: &str) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| game.get_genre() == Some(genre))
            .collect()
    }
    pub fn get_games_with_publisher(&self, publisher: &str) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| game.get_publisher() == Some(publisher))
            .collect()
    }
    pub fn get_games_with_developer(&self, developer: &str) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| game.get_developer() == Some(developer))
            .collect()
    }
    pub fn get_games_with_platform(&self, platform: &str) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| game.get_platform() == Some(platform))
            .collect()
    }
    pub fn get_games_with_rating_in_range(&self, rating_range: std::ops::Range<Rating>) -> Vec<&Game> {
        self.games
            .iter()
            .filter(|game| {
                game.get_personal_rating()
                    .is_some_and(|rating| rating_range.contains(&rating))
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.games.len()
    }

    // updating games (setters)
    /// index, new_title
    pub fn update_game_title_at_index(&mut self, index: usize, new_title: Option<String>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_title(new_title);

        Ok(())
    }
    pub fn update_game_genre_at_index(&mut self, index: usize, new_genre: Option<String>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_genre(new_genre);

        Ok(())
    }
    pub fn update_game_publisher_at_index(&mut self, index: usize, new_publisher: Option<String>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_publisher(new_publisher);

        Ok(())
    }
    pub fn update_game_developer_at_index(&mut self, index: usize, new_developer: Option<String>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_developer(new_developer);

        Ok(())
    }
    pub fn update_game_platform_at_index(&mut self, index: usize, new_platform: Option<String>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_platform(new_platform);

        Ok(())
    }
    pub fn update_game_personal_rating_at_index(&mut self, index: usize, new_personal_rating: Option<Rating>) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games[index].set_personal_rating(new_personal_rating);

        Ok(())
    }
    pub fn update_name(&mut self, name: Option<String>) {
        self.name = name;
    }
    pub fn update_owner(&mut self, owner: Option<String>) {
        self.owner = owner;
    }

    // remove games
    pub fn remove_game_at_index(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot update game at index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games.remove(index);

        Ok(())
    }

    // adding games
    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }
    pub fn insert_game_at_index(&mut self, index: usize, game: Game) -> Result<(), Box<dyn Error>> {
        if index >= self.games.len() {
            return Err(format!("Cannot insert game into index {} of game collection with {} games", index, self.games.len()).into());
        }

        self.games.insert(index, game);

        Ok(())
    }

    // output
    pub fn print_overview(&self) {
        println!("Game Collection Overview");
        println!("Collection Name\t\t\t:\t{}", self.name.as_deref().unwrap_or("not specified"));
        println!("Collection Owner\t\t:\t{}", self.owner.as_deref().unwrap_or("not specified"));
        println!("Number of Games in Collection\t:\t{}\n", self.games.len());
    }
    pub fn print_games(&self) {
        for i in 0..self.games.len() {
            println!("Game Collection Number {}", i);
            self.games[i].print();
        }
    }
    pub fn print_game_at_index(&self, index: usize) {
        self.games[index].print();
    }
}
