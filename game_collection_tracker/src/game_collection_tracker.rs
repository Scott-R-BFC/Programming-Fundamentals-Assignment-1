use crate::game::Game;
use crate::game_collection::GameCollection;
use crate::rating::Rating;

use std::error::Error;

// constant definitions
const ABORT_COMMAND: &str = "abort!";
const ALL_COMMAND: &str = "all!";
const MAX_INPUT_ATTEMPTS: usize = 3;

/// result of input function which can provide some value, no value or cause the calling function to
/// abort operation
enum Input<T> {
    Value(Option<T>),
    Abort,
}

/// types of function which provide an input associated with a `Game` attribute
enum InputFunction {
    String(fn(&str) -> Result<Input<String>, Box<dyn Error>>),
    Rating(fn(&str) -> Result<Input<Rating>, Box<dyn Error>>),
}

/// types of `GameCollection` method which update a `Game`'s attribute at a given index in the
/// collection
enum UpdateFunction {
    String(fn(&mut GameCollection, usize, Option<String>) -> Result<(), Box<dyn Error>>),
    Rating(fn(&mut GameCollection, usize, Option<Rating>) -> Result<(), Box<dyn Error>>),
}

/// types of `GameCollection` method which get all games in the collection associated with a given
/// attribute
enum GetGamesWithAttributeFunction {
    String(for<'a> fn(&'a GameCollection, &str) -> Option<Vec<&'a Game>>),
    Rating(for<'a> fn(&'a GameCollection, std::ops::Range<Rating>) -> Option<Vec<&'a Game>>),
}

/// contains data describing a `Game` attribute and its associated functions for inputting, updating
/// and retrieving `Game`s with that matching attribute from a collection
struct AttributeData {
    name: &'static str,
    input_function: InputFunction,
    update_function: UpdateFunction,
    get_games_function: GetGamesWithAttributeFunction,
}

/// contains data describing attributes which can be updated and viewed in the `menu_loop`
const ATTRIBUTES: &[AttributeData] = &[
    AttributeData {
        name: "title",
        input_function: InputFunction::String(input_string),
        update_function: UpdateFunction::String(GameCollection::update_game_title_at_index),
        get_games_function: GetGamesWithAttributeFunction::String(GameCollection::get_games_with_title),
    },
    AttributeData {
        name: "genre",
        input_function: InputFunction::String(input_string),
        update_function: UpdateFunction::String(GameCollection::update_game_genre_at_index),
        get_games_function: GetGamesWithAttributeFunction::String(GameCollection::get_games_with_genre),
    },
    AttributeData {
        name: "publisher",
        input_function: InputFunction::String(input_string),
        update_function: UpdateFunction::String(GameCollection::update_game_publisher_at_index),
        get_games_function: GetGamesWithAttributeFunction::String(GameCollection::get_games_with_publisher),
    },
    AttributeData {
        name: "developer",
        input_function: InputFunction::String(input_string),
        update_function: UpdateFunction::String(GameCollection::update_game_developer_at_index),
        get_games_function: GetGamesWithAttributeFunction::String(GameCollection::get_games_with_developer),
    },
    AttributeData {
        name: "platform",
        input_function: InputFunction::String(input_string),
        update_function: UpdateFunction::String(GameCollection::update_game_platform_at_index),
        get_games_function: GetGamesWithAttributeFunction::String(GameCollection::get_games_with_platform),
    },
    AttributeData {
        name: "personal rating",
        input_function: InputFunction::Rating(input_rating),
        update_function: UpdateFunction::Rating(GameCollection::update_game_personal_rating_at_index),
        get_games_function: GetGamesWithAttributeFunction::Rating(GameCollection::get_games_with_rating_in_range),
    },
];

/// types of method to be called on the `GameCollectionTracker` in the `menu_loop`
enum Operation {
    Exit,
    Immutable(fn(&GameCollectionTracker) -> Result<(), Box<dyn Error>>),
    Mutable(fn(&mut GameCollectionTracker) -> Result<(), Box<dyn Error>>),
}

/// contains data describing an operation which can be selected in the `menu_loop`
struct OperationData {
    shortcut: &'static str,
    description: &'static str,
    operation: Operation,
}

/// contains data describing operations which can be selected in the `menu_loop`
const OPERATIONS: &[OperationData] = &[
    OperationData {
        shortcut: "q",
        description: "exit program",
        operation: Operation::Exit,
    },
    OperationData {
        shortcut: "vco",
        description: "view collection overview",
        operation: Operation::Immutable(GameCollectionTracker::view_overview),
    },
    OperationData {
        shortcut: "ucn",
        description: "update collection name",
        operation: Operation::Mutable(GameCollectionTracker::update_collection_name),
    },
    OperationData {
        shortcut: "uco",
        description: "update collection owner",
        operation: Operation::Mutable(GameCollectionTracker::update_collection_owner),
    },
    OperationData {
        shortcut: "vag",
        description: "view all games",
        operation: Operation::Immutable(GameCollectionTracker::view_all_games),
    },
    OperationData {
        shortcut: "vga",
        description: "view games with attribute",
        operation: Operation::Immutable(GameCollectionTracker::view_games_with_attribute),
    },
    OperationData {
        shortcut: "add",
        description: "add game",
        operation: Operation::Mutable(GameCollectionTracker::add_game),
    },
    OperationData {
        shortcut: "upd",
        description: "update game",
        operation: Operation::Mutable(GameCollectionTracker::update_game),
    },
    OperationData {
        shortcut: "rem",
        description: "remove game",
        operation: Operation::Mutable(GameCollectionTracker::remove_game),
    },
];

/// create the effect of a cleared console with an ANSI escape sequences
///
/// (clear console & move cursor to start)
fn clear_console() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}

/// read `stdin` into a throwaway `String` to prevent clearing console before user is ready to
/// continue
fn await_return() -> Result<(), Box<dyn Error>> {
    println!("");
    println!("Press Return to Continue.");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

fn input_string(prompt_str: &str) -> Result<Input<String>, Box<dyn Error>> {
    let mut out_string: String = String::new();

    print!("{}", prompt_str);

    // output contents of buffer to console
    // stdout is automatically buffered on `\n`
    // however using `print!` instead of `println!` (which inserts a `\n` and flushes the buffer)
    // allows the input to be on the same line as the prompt
    std::io::Write::flush(&mut std::io::stdout()).map_err(|e| format!("Input aborted. Failed to flush terminal buffer. {e}"))?;

    std::io::stdin()
        .read_line(&mut out_string)
        .map_err(|e| format!("Input aborted. Failed to read input. {e}"))?;

    let out_string = out_string.trim().to_string();

    if out_string == ABORT_COMMAND {
        return Ok(Input::Abort);
    }
    if out_string == "" {
        return Ok(Input::Value(None));
    }

    Ok(Input::Value(Some(out_string)))
}

/// attempts to convert value from `input_string()` to `Rating`
///
/// until conversion fails a number of times greater than `MAX_INPUT_ATTEMPTS`
///
/// or `ABORT_COMMAND` is entered
///
/// in which case `Input::Abort` is returned
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

/// attempts to convert value from `input_string()` to `usize` within a given range
///
/// until conversion fails a number of times greater than `MAX_INPUT_ATTEMPTS`
///
/// or `ABORT_COMMAND` is entered
///
/// in which case `Input::Abort` is returned
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

/// contains a `GameCollection`
pub struct GameCollectionTracker {
    game_collection: GameCollection,
}

/// provides methods for the following:
/// - constructing new `GameCollectionTracker`
/// - selecting data for `Operations` selected in `main_loop`
impl GameCollectionTracker {
    // constructor
    pub fn new() -> GameCollectionTracker {
        GameCollectionTracker {
            game_collection: GameCollection::new(),
        }
    }

    // input collection data
    /// prompts user to select an index associated with a `Game` until either
    /// - successfully selected index
    /// - unsucessfully selected index a number of times greater than `MAX_INPUT_ATTEMPTS`
    /// - a `std::io` error occurs when inputting the index to be selected
    fn input_select_game_index(&self) -> Result<Option<usize>, Box<dyn Error>> {
        let mut select_game_prompt_count = 0;

        let n_games = self.game_collection.len();
        if n_games == 0 {
            println!("Cannot Select Game. No Games.");
            return Ok(None);
        }

        loop {
            println!("Select Game.");
            println!("To abort, enter \"{}\".", ABORT_COMMAND);
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

    /// prompts user to select a `Game` attribute until either
    /// - successfully selected `Game` attribute
    /// - unsucessfully selected `Game` attribute a number of times greater than `MAX_INPUT_ATTEMPTS`
    /// - a `std::io` error occurs when inputting the attribute to be selected
    fn input_select_attribute(game: Option<(&GameCollection, usize)>) -> Result<Option<String>, Box<dyn Error>> {
        let mut select_attribute_prompt_count: usize = 0;

        loop {
            println!("Select Attribute:\n");
            println!("To select all attributes, enter \"{}\".", ALL_COMMAND);
            println!("To abort, enter \"{}\".", ABORT_COMMAND);
            println!("Attribute...");
            println!("Number\tName\t\t:\tValue");

            match game {
                Some((collection, game_index)) => collection.print_game_at_index(game_index),
                None => Game::print_empty(),
            }

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

    /// prompts user to select an `Operation` until either
    /// - successfully selected an `Operation`
    /// - unsuccessfully selected an `Operation` a number of times greater than `MAX_INPUT_ATTEMPTS`
    /// - a `std::io` error occurs when inputting the `Operation` to be selected
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
    /// prompts user to enter a value for each of the `Game` attributes to be added to the
    ///
    /// `GameCollection` until either
    /// - all attribute values successfully entered
    /// - the `ABORT_COMMAND` is entered
    /// - a `std::io` error occurs when inputting an attribute value
    pub fn add_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Entering game details:");
        println!("To abort, enter \"{}\".", ABORT_COMMAND);

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

    /// prompts user to select a `Game` in the `GameCollection`
    ///
    /// then prompts user to select a `Game` attribute
    ///
    /// then prompts user to input a new value for the selected attribute
    ///
    /// then updates the selected attribute
    pub fn update_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Updating Game Attribute");
        let game_index: usize = match self.input_select_game_index()? {
            Some(v) => v,
            None => {
                println!("Game update aborted.");
                return Ok(());
            }
        };

        let input: String = match GameCollectionTracker::input_select_attribute(Some((&self.game_collection, game_index)))? {
            Some(v) => v.to_lowercase(),
            None => {
                print_abort_operation();
                return Ok(());
            }
        };

        println!("To abort, enter \"{}\".", ABORT_COMMAND);
        let update_all_attributes = input == ALL_COMMAND;

        for (i, attribute) in ATTRIBUTES.iter().enumerate() {
            if !update_all_attributes && input != i.to_string() && input != attribute.name {
                continue;
            }

            match attribute.update_function {
                UpdateFunction::String(update_function) => update_function(
                    &mut self.game_collection,
                    game_index,
                    match attribute.input_function {
                        InputFunction::String(input_function) => match input_function(format!("Enter updated {}: ", attribute.name).as_str())? {
                            Input::Value(v) => v,
                            Input::Abort => {
                                print_abort_operation();
                                return Ok(());
                            }
                        },
                        _ => {
                            print_abort_operation();
                            return Err("Only InputFunction::String should be called to update string attribute".into());
                        }
                    },
                )?,
                UpdateFunction::Rating(update_function) => update_function(
                    &mut self.game_collection,
                    game_index,
                    match attribute.input_function {
                        InputFunction::Rating(input_function) => match input_function(format!("Enter updated {}: ", attribute.name).as_str())? {
                            Input::Value(v) => v,
                            Input::Abort => {
                                print_abort_operation();
                                return Ok(());
                            }
                        },
                        _ => {
                            print_abort_operation();
                            return Err("Only InputFunction::Rating should be called to update rating attribute".into());
                        }
                    },
                )?,
            }
        }

        Ok(())
    }

    /// prompts user to select a `Game` in the `GameCollection`
    /// then removes the game from the collection if it exists
    pub fn remove_game(&mut self) -> Result<(), Box<dyn Error>> {
        println!("");
        println!("Removing Game");
        let game_index: usize = match self.input_select_game_index()? {
            Some(v) => v,
            None => {
                print_abort_operation();
                return Ok(());
            }
        };

        self.game_collection.remove_game_at_index(game_index)?;

        Ok(())
    }

    /// prompts user to input a new name for the `GameCollection`
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

    /// prompts user to input a new owner for the `GameCollection`
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
    /// prompts user to select an `Operation` to be performed
    ///
    /// informs the user if their input doesn't select an `Operation`
    ///
    /// leads to the end of the program if `Operation::Exit` is selected
    ///
    /// otherwise performs the Operation
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
                if input != i.to_string() && input != operation_data.shortcut && input != operation_data.description {
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
    /// outputs the number, shortcut and description of each `Operation`
    fn print_menu(&self) {
        println!("Game Collection Tracker Menu");
        println!("-------------------------------------");
        println!("Operation...");
        println!("Number.\tShortcut\tDescription");
        for (i, operation_data) in OPERATIONS.iter().enumerate() {
            println!("{}.\t{}\t{}", i, operation_data.shortcut, operation_data.description);
        }
    }

    /// outputs the `GameCollection` overvieew
    pub fn view_overview(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        self.game_collection.print_overview();

        Ok(())
    }

    /// outputs each `Game` in the `GameCollection`
    pub fn view_all_games(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        self.game_collection.print_games();

        Ok(())
    }

    /// prompts user to select an attribute
    ///
    /// then prompts user to input an attribute value(s) to select `Game`'s who's attributes match the
    /// inputted value(s)
    ///
    /// outputs each `Game` in the `GameCollection` where the `Game` has attributes matching the
    /// user's selection
    pub fn view_games_with_attribute(&self) -> Result<(), Box<dyn Error>> {
        println!("Viewing Games with Selected Attribute");

        let input: String = match GameCollectionTracker::input_select_attribute(None)? {
            Some(v) => v.to_lowercase(),
            None => {
                print_abort_operation();
                return Ok(());
            }
        };

        let mut valid_games: Vec<&Game> = Vec::new();

        if input == ALL_COMMAND {
            self.view_all_games()?;
            return Ok(());
        }

        for (i, attribute) in ATTRIBUTES.iter().enumerate() {
            if input != i.to_string() && input != attribute.name {
                continue;
            }

            let games_with_attribute: Option<Vec<&Game>> = match attribute.get_games_function {
                GetGamesWithAttributeFunction::String(get_games) => {
                    let value = match input_string("Enter Attribute Value: ")? {
                        Input::Value(Some(v)) => v,
                        _ => {
                            print_abort_operation();
                            return Ok(());
                        }
                    };

                    get_games(&self.game_collection, &value)
                }
                GetGamesWithAttributeFunction::Rating(get_games) => {
                    let lower_bound = match input_rating("Enter Rating Lower Bound: ")? {
                        Input::Value(Some(v)) => v,
                        _ => {
                            print_abort_operation();
                            return Ok(());
                        }
                    };

                    let upper_bound = match input_rating("Enter Rating Upper Bound: ")? {
                        Input::Value(Some(v)) => v,
                        _ => {
                            print_abort_operation();
                            return Ok(());
                        }
                    };

                    get_games(&self.game_collection, lower_bound..upper_bound)
                }
            };

            match games_with_attribute {
                Some(games) => valid_games.extend(games),
                None => {
                    println!("No Games with Matching Attribute");
                }
            }
        }

        GameCollection::print_game_slice(&valid_games);

        println!("{}", input);
        Ok(())
    }
}
