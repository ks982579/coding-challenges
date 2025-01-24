#include "binary_search.h"
#include "testframework.h"
#include <iostream>
#include <vector>

int main() {
  std::cout << "Testing ..." << std::endl;
  TestFramework tf;
  Solution sl;
  std::vector<int> x({1, 2, 34});
  sl.search(x, 2);

  return 0;
}
