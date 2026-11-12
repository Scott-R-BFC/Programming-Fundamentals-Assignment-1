#pragma once

#include "includes.h"
#include "Rating.h"

class Game {
private:
   std::string title;
   std::string genre;
   float personal_rating;

public:
   // constructor
   Game(const std::string& in_title, const std::string& in_genre, const float in_personal_rating);

   // getters
   const std::string& get_title() const;
   const std::string& get_genre() const;
   float get_personal_rating() const;

   // setters
   void set_title(const std::string& in_title);
   void set_genre(const std::string& in_genre);
   bool set_personal_rating(const Rating in_personal_rating);

   // output
   void print();

   // static methods
   static bool is_valid_personal_rating(const float personal_rating);
};
