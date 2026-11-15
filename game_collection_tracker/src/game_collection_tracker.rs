use crate::game::Game;
use crate::game_collection::GameCollection;
use crate::rating::Rating;

use std::error::Error;

const INPUT_ABORT_STR: &str = "abort!";
const UPDATE_ALL_ATTRIBUTES_STR: &str = "all!";
const MAX_INPUT_ATTEMPTS: usize = 3;

enum Operation {
    Exit,
    Immutable(fn(&GameCollectionTracker) -> Result<(), Box<dyn Error>>),
    Mutable(fn(&mut GameCollectionTracker) -> Result<(), Box<dyn Error>>),
}

struct OperationData {
    shortcut: &'static str,
    description: &'static str,
    operation: Operation,
}

enum InputFunction {
    StringInput(fn(&str) -> Result<Input<String>, Box<dyn Error>>),
    RatingInput(fn(&str) -> Result<Input<Rating>, Box<dyn Error>>),
}
enum UpdateFunction {
    StringUpdate(fn(&mut GameCollection, usize, Option<String>) -> Result<(), Box<dyn Error>>),
    RatingUpdate(fn(&mut GameCollection, usize, Option<Rating>) -> Result<(), Box<dyn Error>>),
}

struct AttributeData {
    name: &'static str,
    input_function: InputFunction,
    update_function: UpdateFunction,
}

const ATTRIBUTES: &[AttributeData] = &[
    AttributeData { name: "title", input_function: InputFunction::StringInput(input_string), update_function: UpdateFunction::StringUpdate(GameCollection::update_game_title_at_index) },
    AttributeData { name: "genre", input_function: InputFunction::StringInput(input_string), update_function: UpdateFunction::StringUpdate(GameCollection::update_game_genre_at_index) },
    AttributeData { name: "publisher", input_function: InputFunction::StringInput(input_string), update_function: UpdateFunction::StringUpdate(GameCollection::update_game_publisher_at_index) },
    AttributeData { name: "developer", input_function: InputFunction::StringInput(input_string), update_function: UpdateFunction::StringUpdate(GameCollection::update_game_developer_at_index) },
    AttributeData { name: "platform", input_function: InputFunction::StringInput(input_string), update_function: UpdateFunction::StringUpdate(GameCollection::update_game_platform_at_index) },
    AttributeData { name: "personal rating", input_function: InputFunction::RatingInput(input_rating), update_function: UpdateFunction::RatingUpdate(GameCollection::update_game_personal_rating_at_index) },
];

const OPERATIONS: &[OperationData] = &[
    OperationData { shortcut: "q",   description: "exit program",              operation: Operation::Exit },
    OperationData { shortcut: "vco", description: "view collection overview",  operation: Operation::Immutable(GameCollectionTracker::view_overview) }, 
    OperationData { shortcut: "ucn", description: "update collection name",    operation: Operation::Mutable(GameCollectionTracker::update_collection_name) },
    OperationData { shortcut: "uco", description: "update collection owner",   operation: Operation::Mutable(GameCollectionTracker::update_collection_owner) },
    OperationData { shortcut: "vag", description: "view all games",            operation: Operation::Immutable(GameCollectionTracker::view_all_games) },
    OperationData { shortcut: "vga", description: "view games with attribute", operation: Operation::Immutable(GameCollectionTracker::view_games_with_attribute) },
    OperationData { shortcut: "add", description: "add game",                  operation: Operation::Mutable(GameCollectionTracker::add_game) },
    OperationData { shortcut: "upd", description: "update game",               operation: Operation::Mutable(GameCollectionTracker::update_game) },
    OperationData { shortcut: "rem", description: "remove game",               operation: Operation::Mutable(GameCollectionTracker::remove_game) },
];

enum Input<T> {
    Value(Option<T>),
    Abort,
}

fn clear_console() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}

fn await_return() -> Result<(), Box<dyn Error>> {
    println!("");
    println!("Press Return to Continue.");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
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

fn input_usize_in_range(prompt_str: &str, range: std::ops::Range<usize>) -> Result<Input<usize>, Box<dyn Error>> {
    let mut attempt_counter: usize = 0;

    while attempt_counter < MAX_INPUT_ATTEMPTS {
        match input_string(prompt_str)? {
            Input::Abort => return Ok(Input::Abort),
            Input::Value(None) => println!("Input is invalid, empty."),
            Input::Value(Some(input_string)) => match input_string.parse::<usize>() {
                Ok(value) => {
                    if range.contains(&value) {
                        return Ok(Input::Value(Some(value)));
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
    Ok(Input::Abort)
}

fn print_abort_operation() {
    println!("Aborting Operation");
}

pub struct GameCollectionTracker {
    game_collection: GameCollection,
}

impl GameCollectionTracker {
    // constructor
    pub fn new() -> GameCollectionTracker {
        GameCollectionTracker {
            game_collection: GameCollection::new(),
        }
    }

    // input collection data
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
                Input::Abort => {
                    return Ok(None);
                }
                Input::Value(Some(value)) => return Ok(Some(value)),
                Input::Value(None) => {
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

    // update collection data
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
        let update_all_attributes = input == UPDATE_ALL_ATTRIBUTES_STR;

        for (i, attribute) in ATTRIBUTES.iter().enumerate() {
            if !update_all_attributes &&
                input != i.to_string() && 
                input != attribute.name {
                    continue;
            }

            match attribute.update_function {
                UpdateFunction::StringUpdate(update_function) => update_function(
                    &mut self.game_collection,
                    game_index,
                    match attribute.input_function {
                        InputFunction::StringInput(input_function) => match input_function(format!("Enter updated {}: ", attribute.name).as_str())? {
                            Input::Value(v) => v,
                            Input::Abort => {
                                print_abort_operation();
                                return Ok(());
                            }
                        },
                        _ => {
                            print_abort_operation();
                            return Err("Input function other than string input called to update string attribute".into());
                        }
                    }
                )?,
                UpdateFunction::RatingUpdate(update_function) => update_function(
                    &mut self.game_collection,
                    game_index,
                    match attribute.input_function {
                        InputFunction::RatingInput(input_function) => match input_function(format!("Enter updated {}: ", attribute.name).as_str())? {
                            Input::Value(v) => v,
                            Input::Abort => {
                                print_abort_operation();
                                return Ok(());
                            }
                        },
                        _ => {
                            print_abort_operation();
                            return Err("Input function other than string input called to update string attribute".into());
                        }
                    }
                )?,
            }

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

    pub fn update_collection_name(&mut self) -> Result<(), Box<dyn Error>> {
        self.view_overview()?;

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
        self.view_overview()?;

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
    pub fn menu_loop(&mut self) -> Result<(), Box<dyn Error>> {
        'main: loop {
            println!("");
            clear_console();

            self.print_menu();
            println!("Select Operation.");

            let input: String = match input_string("Enter Operation Number, Shortcut or Description: ")? {
                Input::Value(Some(v)) => v.to_lowercase(),
                _ => String::new(),
            };

            let mut valid_input: bool = false;

            for (i, operation_data) in OPERATIONS.iter().enumerate() {
                if input != i.to_string()
                    && input != operation_data.shortcut 
                        && input != operation_data.description { 
                            continue; 
                }

                valid_input = true;

                match operation_data.operation {
                    Operation::Exit => break 'main,
                    Operation::Immutable(op) => op(self)?,
                    Operation::Mutable(op) => op(self)?,
                }

            }

            if !valid_input {
                println!("Invalid Operation Selection");
            }

            await_return()?;
        }

        Ok(())
    }

    // output
    fn print_menu(&self) {
        println!("Game Collection Tracker Menu");
        println!("-------------------------------------");
        println!("Operation...");
        println!("Number.\tShortcut\tDescription");
        for (i, operation_data) in OPERATIONS.iter().enumerate() {
            println!("{}.\t{}\t{}", i, operation_data.shortcut, operation_data.description);
        }

    }

    pub fn view_overview(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        self.game_collection.print_overview();

        Ok(())
    }

    pub fn view_all_games(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        self.game_collection.print_games();

        Ok(())
    }

    pub fn view_games_with_attribute(&self) -> Result<(), Box<dyn Error>> {
        println!("\nNot Implemented Yet.");

        Ok(())
    }
}
