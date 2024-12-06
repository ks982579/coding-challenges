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
    std::cout << "Successfully read " << data.size() << " bytes from data.db"
              << std::endl;
    // Work with Data
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }
  std::cout << data << std::endl;

  Locations locations = Locations::from_string(data);
  locations.sort_locations();
  int all_diffs = locations.sum_diff();
  std::cout << "SUM: " << all_diffs << std::endl;

  locations.clear_cache();
  int sim_score = locations.similarity();
  std::cout << "Similarity Score: " << sim_score << std::endl;

  return 0;
}
