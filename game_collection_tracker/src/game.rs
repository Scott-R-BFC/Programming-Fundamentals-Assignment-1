use crate::rating::Rating;

const EMPTY_GAME_ATTRIBUTE_STR: &str = "not specified";

/// Stores game record information
///
/// Member variables are optional
pub struct Game {
    title: Option<String>,
    genre: Option<String>,
    publisher: Option<String>,
    developer: Option<String>,
    platform: Option<String>,
    personal_rating: Option<Rating>,
}

/// Implements methods for the `Game` class
///
/// Includes methods for:
/// - constructing Game objects
/// - getting Game object member variables
/// - setting Game object member variables
/// - outputting Game object data
impl Game {
    // constructors
    /// returns Game object with provided attributes
    pub fn new(title: Option<String>, genre: Option<String>, publisher: Option<String>, developer: Option<String>, platform: Option<String>, personal_rating: Option<Rating>) -> Game {
        Game {
            title,
            genre,
            platform,
            publisher,
            developer,
            personal_rating,
        }
    }
    /// returns Game object with all attributes None
    pub fn empty() -> Game {
        Game {
            title: None,
            genre: None,
            publisher: None,
            developer: None,
            platform: None,
            personal_rating: None,
        }
    }

    // getters
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    pub fn get_genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }
    pub fn get_platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }
    pub fn get_publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }
    pub fn get_developer(&self) -> Option<&str> {
        self.developer.as_deref()
    }
    pub fn get_personal_rating(&self) -> Option<Rating> {
        self.personal_rating
    }

    // setters
    pub fn set_title(&mut self, new_title: Option<String>) {
        self.title = new_title;
    }
    pub fn set_genre(&mut self, new_genre: Option<String>) {
        self.genre = new_genre;
    }
    pub fn set_platform(&mut self, new_platform: Option<String>) {
        self.platform = new_platform;
    }
    pub fn set_publisher(&mut self, new_publisher: Option<String>) {
        self.publisher = new_publisher;
    }
    pub fn set_developer(&mut self, new_developer: Option<String>) {
        self.developer = new_developer;
    }
    pub fn set_personal_rating(&mut self, new_personal_rating: Option<Rating>) {
        self.personal_rating = new_personal_rating;
    }

    // output
    /// outputs Game object's attribute's value or a constant default [EMPTY_GAME_ATTRIBUTE_STR]
    pub fn print(&self) {
        println!("0\tTitle\t\t:\t{}", self.title.as_deref().unwrap_or(EMPTY_GAME_ATTRIBUTE_STR));
        println!("1\tGenre\t\t:\t{}", self.genre.as_deref().unwrap_or(EMPTY_GAME_ATTRIBUTE_STR));
        println!("2\tPublisher\t:\t{}", self.publisher.as_deref().unwrap_or(EMPTY_GAME_ATTRIBUTE_STR));
        println!("3\tDeveloper\t:\t{}", self.developer.as_deref().unwrap_or(EMPTY_GAME_ATTRIBUTE_STR));
        println!("4\tPlatform\t:\t{}", self.platform.as_deref().unwrap_or(EMPTY_GAME_ATTRIBUTE_STR));
        print!("5\tPersonal Rating\t:\t");

        match self.personal_rating {
            Some(r) => {
                println!("{}", r)
            }
            None => {
                println!("{}", EMPTY_GAME_ATTRIBUTE_STR)
            }
        }
    }

    /// outputs attributes of empty Game object
    pub fn print_empty() {
        println!("0\tTitle\t\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
        println!("1\tGenre\t\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
        println!("2\tPublisher\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
        println!("3\tDeveloper\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
        println!("4\tPlatform\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
        println!("5\tPersonal Rating\t:\t{}", EMPTY_GAME_ATTRIBUTE_STR);
    }
}
