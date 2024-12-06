// src/libs/assets_reader.cpp

#include "locations.h"
#include <algorithm>
#include <cstddef>
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

void Locations::sort_locations() {
  // https://cplusplus.com/reference/algorithm/sort/?kw=sort
  std::sort(this->m_left.begin(), this->m_left.end(),
            [](int a, int b) { return a >= b; });
  std::sort(this->m_right.begin(), this->m_right.end(),
            [](int a, int b) { return a >= b; });
}

int Locations::sum_diff() {
  // https://cplusplus.com/forum/general/194034/
  // size_t is not int - different bytes and unsigned
  size_t elms{this->m_left.size()};
  int the_sum = 0;
  for (size_t i = 0; i < elms; i++) {
    int left = this->m_left[i];
    int right = this->m_right[i];
    the_sum += std::max(left, right) - std::min(left, right);
  }
  return the_sum;
}

void Locations::clear_cache() { this->m_cache.clear(); }

int Locations::similarity() {
  // Sloppy...
  size_t elms{this->m_left.size()};
  int sim_score = 0;
  for (size_t i = 0; i < elms; i++) {
    int left = this->m_left.at(i);
    // If in cache, get that, else count...
    // if(this->m_cache.contains(left)) { // not sure if it is real
    // could also remove values in vector... but that would be a lot of work.
    if (this->m_cache.find(left) != this->m_cache.end()) {
      sim_score += this->m_cache.at(left) * left;
    } else {
      // Brute force count
      int tmp = 0;
      for (size_t j = 0; j < elms; j++) {
        tmp += (left == this->m_right.at(j)) ? 1 : 0;
      }
      this->m_cache.insert({left, tmp});
      sim_score += left * tmp;
    }
  }
  return sim_score;
}
