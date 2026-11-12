#include "Game.h"

// constructor
Game::Game(const std::string& in_title, const std::string& in_genre , const float in_personal_rating)
: title(in_title), genre(in_genre) {
    if (!Game::is_valid_personal_rating(in_personal_rating)) {
        personal_rating = 0.0;
    } else {
        personal_rating = in_personal_rating;
    }
}

// getters
const std::string& Game::get_title() const {
    return title;
}

const std::string& Game::get_genre() const {
    return genre;
}

float Game::get_personal_rating() const {
    return personal_rating;
}

// setters
void Game::set_title(const std::string& in_title) {
    title = in_title;
}

void Game::set_genre(const std::string& in_genre) {
    genre = in_genre;
}

// output 
void Game::print() {
    std::cout << "Game Title: " << title << "\n";
    std::cout << "Game Genre: " << genre << "\n";
    std::cout << "Game Personal Rating: " << personal_rating << "\n";
}

// static methods
static bool is_valid_personal_rating(const float in_personal_rating) {
    return in_personal_rating >= 0.0 && in_personal_rating <= 10.00;
}


