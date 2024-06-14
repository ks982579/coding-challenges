#include <algorithm> // for sorting
#include <iostream>
#include <tuple>
#include <vector>

/* Have fun with this
 * find the largest number in list that is smaller than output
 * >> could be weird binary search, requires sorting
 * In remaining list, search for compatible number.
 * */

class Computer {
private:
  std::vector<int> numbers;
  int calc_len() { return sizeof(numbers) / sizeof(int); }

public:
  int length;
  Computer(const std::vector<int> &data) {
    numbers = data; // hopefully performs copy
    std::sort(numbers.begin(), numbers.end());
    length = calc_len();
  }

  // index_closest_small_value()
  unsigned int index_closest_small_value() { return 42; }

  // two_sum()
  std::tuple<int, int> two_sum(int a, int b) {
    size_t ind = index_closest_small_value();
    return std::make_tuple(0, 0);
  }
};

void tests() {
  std::vector<int> list = {10, 13, 22, 55, 42};
  Computer cmp = Computer(list);
}

// return indices of two numbers that sum to target.
int main() {
  std::vector<int> list = {2,   3,   4,   7,   11,  13,  17,  19,  13,  29,
                           31,  37,  41,  43,  47,  53,  59,  61,  67,  71,
                           73,  79,  83,  89,  97,  101, 103, 107, 109, 113,
                           127, 131, 137, 139, 149, 151, 157, 163, 167, 173};
  // Creates copy... Chaning ownership may optimize.
  Computer comp = Computer(list);
  std::cout << "Hello, world";
  return 0;
}
