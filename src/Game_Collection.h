#pragma once

#include "includes.h"
#include "Game.h"

class Game_Collection {
private:
    std::vector<Game> games;
public:
    Game_Collection();

    void insert_game(Game game);

    std::vector<const Game*> get_games_by_title(const std::string& title) const; 
    std::vector<const Game*> get_games_by_genre(const std::string& genre) const; 
    std::vector<const Game*> get_games_by_rating_range(float lower) const; // finish after making rating range type

};
