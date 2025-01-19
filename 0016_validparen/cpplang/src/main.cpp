#include "libs/asset_reader.h"
#include <exception>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <stack>
#include <string>

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

    // work
    std::stack<char> validator;
    bool isValid = true;

    for (char c : line) {
      if (c == '(' || c == '{' || c == '[') {
        validator.push(c);
      } else if (c == ')' || c == '}' || c == ']') {
        if (validator.empty()) {
          isValid = false;
          break;
        }

        char top = validator.top();
        if ((c == ')' && top == '(') || (c == ']' && top == '[') ||
            (c == '}' && top == '{')) {
          validator.pop();
        } else {
          isValid = false;
          break;
        }
      }
    }

    std::cout << line << " | Is Valid: " << ((isValid) ? "Yes" : "No") << "\n";

    // Update String
    data.erase(0, pos + 1);
  }

  return 0;
}
