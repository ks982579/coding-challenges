#include "libs/asset_reader.h"
#include <exception>
#include <iostream>

int main() {
  try {
    if (!AssetReader::fileExists("data.db")) {
      std::cerr << "Could not find data.db in assets folder" << std::endl;
      return 1;
    }
    auto data = AssetReader::readFileToBinary("data.db");
    std::cout << "Successfully read " << data.size() << " bytes from data.db"
              << std::endl;
    // Work with Data
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }
  return 0;
}
