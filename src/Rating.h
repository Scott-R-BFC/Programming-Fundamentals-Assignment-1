#pragma once 
#include <stdexcept>

class Rating {
private:
    float value;

public:
    Rating(float in_value) {
        if (in_value < 0.0f || in_value > 10.0f) {
            throw std::out_of_range("Rating must be between 0.0 and 10.0");
        }

        value = in_value;
    }
};
