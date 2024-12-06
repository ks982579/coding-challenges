#include "libs/asset_reader.h"
#include "libs/locations.h"
#include <exception>
#include <iostream>
#include <string>

int main() {
  std::string data;
  try {
    if (!AssetReader::fileExists("data.db")) {
      std::cerr << "Could not find data.db in assets folder" << std::endl;
      return 1;
    }
    data = AssetReader::readFileToString("data.db");
    Locations locations = Locations::from_string(data);
    std::cout << "Successfully read " << data.size() << " bytes from data.db"
              << std::endl;
    // Work with Data
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }
  std::cout << data << std::endl;
  return 0;
}
