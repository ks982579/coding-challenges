#include "libs/asset_reader.h"
#include <algorithm>
#include <cstddef>
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

struct Answer {
  int buy;
  int sell;
  int profit;

  void set_schedule(int b, int s, int p) {
    buy = b;
    sell = s;
    profit = p;
  };
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

    // Should initialize to zero
    Answer answer{};
    // std::cout << "ANSWER: " << answer.buy << ", " << answer.sell <<
    // std::endl;

    StockString prices_str(std::move(line));
    std::vector<int> prices = prices_str.to_vec_int();

    // work
    // to prices.size() -1 because can't buy and sell on same day
    // for (size_t i = 0; i < prices.size() - 1; i++) {
    //   std::cout << prices.at(i) << std::endl;
    //   for (size_t j = i + 1; j < prices.size(); j++) {
    //     int ex_profit = prices.at(j) - prices.at(i);
    //     if (ex_profit > answer.profit) {
    //       answer.set_schedule(prices.at(i), prices.at(j), ex_profit);
    //     }
    //   }
    // }

    int tmp_buy{prices.at(0)};

    // Following Kadane's approach
    // loop through only once, since same direction is ok
    for (size_t i = 0; i < prices.size(); i++) {
      std::cout << prices.at(i) << std::endl;
      // If we have a new lowest price, we consider buying from this point
      if (prices.at(i) < tmp_buy) {
        tmp_buy = prices.at(i);
        // If buying at new price produces greater profit, store result
      } else if (prices.at(i) - tmp_buy > answer.profit) {
        answer.set_schedule(tmp_buy, prices.at(i), prices.at(i) - tmp_buy);
      }
    }
    std::cout << "ANSWER: " << answer.buy << ", " << answer.sell << ", "
              << answer.profit << std::endl;

    // Update String
    data.erase(0, pos + 1);
  }

  return 0;
}
