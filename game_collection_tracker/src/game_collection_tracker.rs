use crate::game_collection::GameCollection;
use crate::game::Game;
use crate::rating::Rating;

const INPUT_ABORT_STR: &str = "abort!";

enum Input<T> {
    Value(Option<T>),
    Abort,
}


fn input_string_attribute(prompt_str: &str) -> Result<Input<String>, Box<dyn std::error::Error>> {
    let mut out_string: String = String::new();

    print!("{}", prompt_str);

    std::io::Write::flush(&mut std::io::stdout()).map_err(|e| format!("Fame attribute input aborted. Failed to flush terminal buffer {e}"))?;

    std::io::stdin().read_line(&mut out_string).map_err(|e| format!("Game attributee input aborted. Failed to read input. {e}"))?;

    let out_string = out_string.trim().to_string();

    if out_string == INPUT_ABORT_STR { return Ok(Input::Abort) }
    if out_string == "" { return Ok(Input::Value(None)); }

    Ok(Input::Value(Some(out_string)))
}

fn input_rating_attribute(prompt_str: &str) -> Result<Input<Rating>, Box<dyn std::error::Error>> {
    let mut loop_counter: usize = 0;
    let max_loops: usize = 5;

    while loop_counter < max_loops{
        match input_string_attribute(prompt_str)? {
            Input::Abort                     => return Ok(Input::Abort),
            Input::Value(None)               => return Ok(Input::Value(None)),
            Input::Value(Some(input_string)) => match input_string.parse::<f32>() {
                Ok(value) => match Rating::from(value) {
                    Ok(rating) => return Ok(Input::Value(Some(rating))),
                    Err(_)     => println!("Input is not a valid rating. Rating must be between 0 and 10"),
                },
                Err(_) => println!("Input is not a valid rating. Rating must be numeric"),
            },
        }

        loop_counter  += 1
    }

    println!("Game attribute input aborted. Failed to provide valid input too many times");
    return Ok(Input::Abort);
}

pub struct GameCollectionTracker {
    game_collection: GameCollection,
}

impl GameCollectionTracker {
    pub fn new() -> GameCollectionTracker {
        GameCollectionTracker { game_collection: GameCollection::new() }
    }

    pub fn add_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let title: Option<String> = match input_string_attribute("Enter Game Title: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };
        let genre: Option<String> = match input_string_attribute("Enter Game Genre: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };
        let publisher: Option<String> = match input_string_attribute("Enter Game Publisher: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };
        let developer: Option<String> = match input_string_attribute("Enter Game Developer: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };
        let platform: Option<String> = match input_string_attribute("Enter Game Platform: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };

        let personal_rating: Option<Rating> = match input_rating_attribute("Enter Game Personal Rating: ")? {
            Input::Value(v) => v,
            Input::Abort    => { println!("Game attribute input aborted."); return Ok(()); }
        };

        self.game_collection.add_game(Game::new(title, genre, publisher, developer, platform, personal_rating));

        Ok(())
    }

    pub fn view_all_games(&self) {
        self.game_collection.view_games();
    }
}

