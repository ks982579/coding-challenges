#include "flood_fill.h"
#include <iostream>
#include <queue>
#include <utility>

// using Image = std::vector<std::vector<int>>;

Image Solution::floodFill(Image &image, int sr, int sc, int colour) {
  // Image x = {{0}, {0}};
  // Probably need checks first
  // Requires reference to change.
  std::queue<std::pair<int, int>> qu;
  qu.push({sr, sc});

  int max_rows = image.size();
  int max_cols = image.at(0).size();
  int color_og = image.at(sr).at(sc); // should be a copy

  while (qu.empty() == 0) {
    int row = qu.front().first;
    int col = qu.front().second;

    // Update Pixel
    int &pix = image.at(row).at(col);
    pix = colour;

    // pop from queue
    qu.pop();

    int tmp_color;

    // Update queue
    if (0 <= row + 1 && row + 1 < max_rows) {
      tmp_color = image.at(row + 1).at(col);
      if (tmp_color == color_og) {
        std::cout << "Adding below" << std::endl;
        qu.push({row + 1, col});
      }
    }
    if (0 <= row - 1 && row - 1 < max_rows) {
      tmp_color = image.at(row - 1).at(col);
      if (tmp_color == color_og) {
        std::cout << "Adding above" << std::endl;
        qu.push({row - 1, col});
      }
    }
    if (0 <= col + 1 && col + 1 < max_cols) {
      tmp_color = image.at(row).at(col + 1);
      if (tmp_color == color_og) {
        std::cout << "Adding left" << std::endl;
        qu.push({row, col + 1});
      }
    }
    if (0 <= col - 1 && col - 1 < max_cols) {
      tmp_color = image.at(row).at(col - 1);
      if (tmp_color == color_og) {
        std::cout << "Adding right" << std::endl;
        qu.push({row, col - 1});
      }
    }
  }

  // return value
  return image;
};
