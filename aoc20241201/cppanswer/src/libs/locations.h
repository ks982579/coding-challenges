// ./src/libs/locations.h
#pragma once
#ifndef LOCATIONS_H
#define LOCATIONS_H

#include <string>
#include <tuple>
#include <vector>

class Locations {
public:
  // Constructor that takes two vectors
  Locations(const std::vector<int> &left, const std::vector<int> &right);

  static Locations from_string(const std::string &data);

private:
  // "m" is for member
  std::vector<int> m_left;
  std::vector<int> m_right;
};

#endif

// const before is for return type
// const after is guarantee it doesn't change object.
