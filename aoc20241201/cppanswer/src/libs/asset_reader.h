// ./src/libs/asset_reader.h
#pragma once
#ifndef ASSET_READER_H
#define ASSET_READER_H

#include <fstream>
#include <string>
#include <vector>

class AssetReader {
public:
  // Reads entire file into a string
  static std::string readFileToString(const std::string &filepath);

  // Reads file into binary data
  static std::vector<uint8_t> readFileToBinary(const std::string &filepath);

  static bool fileExists(const std::string &filepath);

private:
  // Get full path of asset file
  static std::string getAssetPath(const std::string &filename);
};

#endif
