// ./src/libs/asset_reader.h

#pragma once
#ifndef ASSET_READER_H
#define ASSET_READER_H

#include <fstream>
#include <string>
#include <vector>

class AssetReader {
public:
  static std::string readFileToString(const std::string &filepath);

private:
  // get full path of asset file
  static std::string getAssetPath(const std::string &filename);
};

#endif
