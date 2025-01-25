// #include <ostream>
#include <vector>

using Image = std::vector<std::vector<int>>;

class Solution {

public:
  static Image floodFill(Image &image, int sr, int sc, int colour);
};

// std::ostream &operator<<(std::ostream &os, const Image &img) {
//   os << "{";
//
//   for (const auto &row : img) {
//     os << "{";
//     for (const auto &pixel : row) {
//       os << pixel << ",";
//     }
//     os << "}";
//   }
//   os << "}";
//   return os;
// }
