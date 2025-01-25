#include "flood_fill.h"
#include <queue>
#include <utility>

// using Image = std::vector<std::vector<int>>;

Image Solution::floodFill(Image &image, int sr, int sc, int colour) {
  // Image x = {{0}, {0}};
  // Probably need checks first
  // Requires reference to change.
  std::queue<std::pair<int, int>> qu;
  qu.push({sr, sc});

  while (qu.empty() == 0) {
    int &pix = image.at(sr).at(sc);
    pix = colour;
  }

  // return value
  return image;
}
