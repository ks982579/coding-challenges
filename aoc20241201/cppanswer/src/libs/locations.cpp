// src/libs/assets_reader.cpp

#include "locations.h"
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

Locations::Locations(const std::vector<int> &left,
                     const std::vector<int> &right) {
  m_left = left;
  m_right = right;
}

Locations Locations::from_string(const std::string &data) {
  std::vector<int> left;
  std::vector<int> right;

  // Because I did not sstream::getline();
  std::istringstream stream{data};

  std::string line;
  while (std::getline(stream, line)) {
    if (line.empty())
      continue;

    std::istringstream converter{line};

    int numA, numB;
    if (converter >> numA >> numB) { // Skips whitespace with stream
      left.push_back(numA);
      right.push_back(numB);
    } else {
      throw std::runtime_error("Invalid Line format in file: " + line);
    }
  }

  return Locations{left, right};
}
