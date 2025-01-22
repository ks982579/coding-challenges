#include "libs/asset_reader.h"
#include <algorithm>
#include <exception>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

// Practice with Inheriting Constructors
class StockString : public std::string {
public:
  // To inherit all constructors from std::string;
  using std::string::string;

  // Compiler using different constructor
  StockString(std::string &&str) : std::string(std::move(str)) {}

  std::vector<int> to_vec_int() const {
    std::vector<int> result;
    std::stringstream ss(*this);
    std::string elm;
    while (std::getline(ss, elm, ',')) {
      result.push_back(std::stoi(elm));
    };
    return result;
  }
};

int main() {
  std::string data;
  try {
    if (!std::filesystem::exists("assets/data.txt")) {
      std::cerr << "COULD NOT FIND FILE" << std::endl;
      return 1;
    }
    data = AssetReader::readFileToString("data.txt");
    std::cout << "Successfully read " << data.size() << " bytes from file"
              << "\n";
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }

  size_t pos = 0;

  while ((pos = data.find('\n')) != std::string::npos) {
    std::string line = data.substr(0, pos);

    if (line.length() < 1) {
      break;
    }

    StockString prices_str(std::move(line));
    std::vector<int> prices = prices_str.to_vec_int();

    // work

    std::cout << prices[0] << std::endl;

    // Update String
    data.erase(0, pos + 1);
  }

  return 0;
}
