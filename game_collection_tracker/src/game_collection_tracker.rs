use crate::game::Game;
use crate::game_collection::GameCollection;
use crate::rating::Rating;

use std::error::Error;

const INPUT_ABORT_STR: &str = "abort!";
const UPDATE_ALL_ATTRIBUTES_STR: &str = "all!";
const MAX_INPUT_ATTEMPTS: usize = 3;

enum Input<T> {
    Value(Option<T>),
    Abort,
}

fn clear_console() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}

fn input_string(prompt_str: &str) -> Result<Input<String>, Box<dyn Error>> {
    let mut out_string: String = String::new();

    print!("{}", prompt_str);

    std::io::Write::flush(&mut std::io::stdout()).map_err(|e| format!("Input aborted. Failed to flush terminal buffer. {e}"))?;

    std::io::stdin()
        .read_line(&mut out_string)
        .map_err(|e| format!("Input aborted. Failed to read input. {e}"))?;

    let out_string = out_string.trim().to_string();

    if out_string == INPUT_ABORT_STR {
        return Ok(Input::Abort);
    }
    if out_string == "" {
        return Ok(Input::Value(None));
    }

    Ok(Input::Value(Some(out_string)))
}

fn input_rating(prompt_str: &str) -> Result<Input<Rating>, Box<dyn Error>> {
    let mut attempt_counter: usize = 0;

    while attempt_counter < MAX_INPUT_ATTEMPTS {
        match input_string(prompt_str)? {
            Input::Abort => return Ok(Input::Abort),
            Input::Value(None) => return Ok(Input::Value(None)),
            Input::Value(Some(input_string)) => match input_string.parse::<f32>() {
                Ok(value) => match Rating::from(value) {
                    Ok(rating) => return Ok(Input::Value(Some(rating))),
                    Err(_) => println!("Input is not a valid rating. Rating must be between 0 and 10."),
                },
                Err(_) => println!("Input is not a valid rating. Rating must be numeric."),
            },
        }

        attempt_counter += 1;
    }

    println!("Failed to provide valid input too many times.");
    Ok(Input::Abort)
}

fn input_usize_in_range(prompt_str: &str, range: std::ops::Range<usize>) -> Result<Option<usize>, Box<dyn Error>> {
    let mut attempt_counter: usize = 0;

    while attempt_counter < MAX_INPUT_ATTEMPTS {
        match input_string(prompt_str)? {
            Input::Abort => return Ok(None),
            Input::Value(None) => println!("Input is invalid, empty."),
            Input::Value(Some(input_string)) => match input_string.parse::<usize>() {
                Ok(value) => {
                    if range.contains(&value) {
                        return Ok(Some(value));
                    } else {
                        println!("Input is invalid, not in range.");
                    }
                }
                Err(_) => println!("Input is invalid, must be non negative integer."),
            },
        }

        attempt_counter += 1;
    }

    println!("Failed to provide valid input too many times.");
    Ok(None)
}

fn print_abort_operation() {
    println!("Aborting Operation");
}

pub struct GameCollectionTracker {
    game_collection: GameCollection,
}

impl GameCollectionTracker {
    pub fn new() -> GameCollectionTracker {
        GameCollectionTracker {
            game_collection: GameCollection::new(),
        }
    }

    pub fn add_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Entering game details:");
        println!("To abort, enter \"{}\".", INPUT_ABORT_STR);

        let title: Option<String> = match input_string("Enter Game Title: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };
        let genre: Option<String> = match input_string("Enter Game Genre: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };
        let publisher: Option<String> = match input_string("Enter Game Publisher: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };
        let developer: Option<String> = match input_string("Enter Game Developer: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };
        let platform: Option<String> = match input_string("Enter Game Platform: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };

        let personal_rating: Option<Rating> = match input_rating("Enter Game Personal Rating: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };

        self.game_collection
            .add_game(Game::new(title, genre, publisher, developer, platform, personal_rating));

        Ok(())
    }

    pub fn update_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Updating Game Attribute");
        let game_index: usize = match self.input_select_game()? {
            Some(v) => v,
            None => {
                println!("Game update aborted.");
                return Ok(());
            }
        };

        let input: String = match self.input_select_attribute(game_index)? {
            Some(v) => v.to_lowercase(),
            None => {
                print_abort_operation();
                return Ok(());
            }
        };

        println!("To abort, enter \"{}\".", INPUT_ABORT_STR);
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "1" || input == "title" {
            self.game_collection.update_game_title_at_index(
                game_index,
                match input_string("Enter updated game title: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "2" || input == "genre" {
            self.game_collection.update_game_genre_at_index(
                game_index,
                match input_string("Enter updated game genre: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "3" || input == "publisher" {
            self.game_collection.update_game_publisher_at_index(
                game_index,
                match input_string("Enter updated game publisher: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "4" || input == "developer" {
            self.game_collection.update_game_developer_at_index(
                game_index,
                match input_string("Enter updated game developer: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "5" || input == "platform" {
            self.game_collection.update_game_platform_at_index(
                game_index,
                match input_string("Enter updated game platform: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }
        if input == UPDATE_ALL_ATTRIBUTES_STR || input == "6" || input == "platform" {
            self.game_collection.update_game_personal_rating_at_index(
                game_index,
                match input_rating("Enter Game Personal Rating: ")? {
                    Input::Value(v) => v,
                    Input::Abort => {
                        print_abort_operation();
                        return Ok(());
                    }
                },
            )?;
        }

        Ok(())
    }

    pub fn remove_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Removing Game");
        let game_index: usize = match self.input_select_game()? {
            Some(v) => v,
            None => {
                print_abort_operation();
                return Ok(());
            }
        };

        self.game_collection.remove_game_at_index(game_index)?;

        Ok(())
    }

    fn input_select_game(&self) -> Result<Option<usize>, Box<dyn Error>> {
        let mut select_game_prompt_count = 0;

        let n_games = self.game_collection.len();
        if n_games == 0 {
            println!("Cannot Select Game. No Games.");
            return Ok(None);
        }

        loop {
            println!("Select Game.");
            println!("To abort, enter \"{}\".", INPUT_ABORT_STR);
            self.game_collection.print_games();

            match input_usize_in_range(format!("Enter Game Collection Number (in the inclusive range {} .. {}): ", 0, n_games - 1).as_str(), 0..n_games)? {
                Some(value) => return Ok(Some(value)),
                None => {
                    select_game_prompt_count += 1;
                    if select_game_prompt_count > MAX_INPUT_ATTEMPTS {
                        println!("Failed to provide valid input too many times");
                        return Ok(None);
                    }
                }
            }
        }
    }

    fn input_select_attribute(&self, game_index: usize) -> Result<Option<String>, Box<dyn Error>> {
        let mut select_attribute_prompt_count: usize = 0;

        loop {
            println!("Select Attribute:\n");
            println!("To select all attributes, enter \"{}\".", UPDATE_ALL_ATTRIBUTES_STR);
            println!("To abort, enter \"{}\".", INPUT_ABORT_STR);
            println!("Attribute...");
            println!("Number\tName\t\t:\tValue");
            self.game_collection.print_game_at_index(game_index);

            match input_string("Enter Attribute Number or Attribute Name: ")? {
                Input::Value(option) => match option {
                    Some(v) => return Ok(Some(v)),
                    None => {
                        select_attribute_prompt_count += 1;
                        if select_attribute_prompt_count > MAX_INPUT_ATTEMPTS {
                            println!("Failed to provide valid input too many times.");
                            return Ok(None);
                        }
                    }
                },
                Input::Abort => {
                    return Ok(None);
                }
            }
        }
    }

    fn input_select_operation(&self) -> Result<String, Box<dyn Error>> {
        self.print_menu();
        println!("Select Operation.");

        match input_string("Enter Operation Number, Shortcut or Description")? {
            Input::Value(option) => match option {
                Some(v) => return Ok(v),
                None => return Ok(String::from("")),
            },
            Input::Abort => return Ok(String::from("")),
        }
    }

    pub fn update_collection_name(&mut self) -> Result<(), Box<dyn Error>> {
        self.view_overview();

        println!("");

        let input: Option<String> = match input_string("Enter new collection name: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };

        self.game_collection.update_name(input);

        Ok(())
    }

    pub fn update_collection_owner(&mut self) -> Result<(), Box<dyn Error>> {
        self.view_overview();

        println!("");

        let input: Option<String> = match input_string("Enter new collection owner: ")? {
            Input::Value(v) => v,
            Input::Abort => {
                print_abort_operation();
                return Ok(());
            }
        };

        self.game_collection.update_owner(input);

        Ok(())
    }

    // io loop
    fn print_menu(&self) {
        println!("Game Collection Tracker Menu");
        println!("-------------------------------------");
        println!("Operation...");
        println!("Number\tShortcut\tDescription");
        println!("0\tq  \t\tExit Program");
        println!("1\tvco\t\tView Collection Overview");
        println!("2\tucn\t\tUpdate Collection Name");
        println!("3\tuco\t\tUpdate Collection Owner");
        println!("4\tvag\t\tView All Games");
        println!("5\tvga\t\tView Games with Attribute");
        println!("6\tadd\t\tAdd Game");
        println!("7\tupd\t\tUpdate Game");
        println!("8\trem\t\tRemove Game");
    }

    pub fn menu_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            println!("");
            clear_console();

            self.print_menu();
            println!("Select Operation.");

            let input: String = match input_string("Enter Operation Number, Shortcut or Description: ")? {
                Input::Value(option) => match option {
                    Some(v) => v,
                    None => String::from(""),
                },
                Input::Abort => String::from(""),
            };

            if input == "0" || input == "q" || input == "exit program" {
                break;
            }
            if input == "1" || input == "vco" || input == "view collection overview" {
                self.view_overview();
            }
            if input == "2" || input == "ucn" || input == "update collection name" {
                self.update_collection_name()?;
            }
            if input == "3" || input == "uco" || input == "update collection owner" {
                self.update_collection_owner()?;
            }
            if input == "4" || input == "vag" || input == "view all games" {
                self.view_all_games();
            }
            if input == "5" || input == "vga" || input == "view games with attribute" {
                self.view_games_with_attribute()?;
            }
            if input == "6" || input == "add" || input == "add game" {
                self.add_game()?;
            }
            if input == "7" || input == "upd" || input == "update game" {
                self.update_game()?;
            }
            if input == "8" || input == "rem" || input == "remove game" {
                self.remove_game()?;
            }

            println!("");
            println!("Press Return to Continue.");
            // await newline input
            std::io::stdin().read_line(&mut String::new())?;
        }

        Ok(())
    }

    // output
    pub fn view_overview(&self) {
        println!("");
        self.game_collection.print_overview();
    }
    pub fn view_all_games(&self) {
        println!("");
        self.game_collection.print_games();
    }
    pub fn view_games_with_attribute(&self) -> Result<(), Box<dyn Error>> {
        println!("\nNot Implemented Yet.");

        Ok(())
    }
}
