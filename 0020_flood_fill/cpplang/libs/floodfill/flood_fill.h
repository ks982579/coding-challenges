#include <vector>

using Image = std::vector<std::vector<int>>;

class Solution {

public:
  static Image floodFill(Image &image, int sr, int sc, int colour);
};
