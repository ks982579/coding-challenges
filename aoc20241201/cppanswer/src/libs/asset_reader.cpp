// src/libs/assets_reader.cpp
#include "asset_reader.h"
#include <cstdint>
#include <filesystem>
#include <ios>
#include <sstream> // std::stringstream, std::stringbuf
#include <stdexcept>
#include <string>
#include <vector>

std::string AssetReader::readFileToString(const std::string &filepath) {
  // Should automatically close files when going out of scope
  // thanks to Resource Acquisition Is Initialization (RAII).
  std::ifstream file(getAssetPath(filepath), std::ios::in);
  if (!file) {
    throw std::runtime_error("Could not open file: " + filepath);
  }
  std::stringstream buffer;
  // https://cplusplus.com/reference/sstream/stringstream/rdbuf/
  buffer << file.rdbuf();
  return buffer.str();
}

std::vector<uint8_t>
AssetReader::readFileToBinary(const std::string &filepath) {
  std::ifstream file(getAssetPath(filepath), std::ios::binary);
  if (!file) {
    throw std::runtime_error("Could not open file: " + filepath);
  }

  // Get File Size
  file.seekg(0, std::ios::end);
  std::streamsize size = file.tellg();
  file.seekg(0, std::ios::beg);

  // Read Data
  std::vector<uint8_t> buffer(size);
  if (!file.read(reinterpret_cast<char *>(buffer.data()), size)) {
    throw std::runtime_error("Failed to read file: " + filepath);
  }

  return buffer;
}

bool AssetReader::fileExists(const std::string &filepath) {
  return std::filesystem::exists(getAssetPath(filepath));
}

std::string AssetReader::getAssetPath(const std::string &filename) {
  return "assets/" + filename;
}
