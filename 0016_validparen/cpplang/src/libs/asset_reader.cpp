// src/libs/asset_reader.cpp
#include "asset_reader.h"
#include <filesystem>
#include <fstream>
#include <sstream> // std::stringstream, std::stringbuf
#include <stdexcept>
#include <string>

std::string AssetReader::readFileToString(const std::string &filepath) {
  std::ifstream file(getAssetPath(filepath), std::ios::in);
  if (!file) {
    throw std::runtime_error("Could not open file: " + filepath);
  }
  std::stringstream buffer;
  buffer << file.rdbuf();
  return buffer.str();
};

std::string AssetReader::getAssetPath(const std::string &filename) {
  return "assets/" + filename;
}
