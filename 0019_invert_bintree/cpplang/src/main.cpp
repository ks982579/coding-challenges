#include "libs/asset_reader.h"
#include <algorithm>
#include <cstddef>
#include <cstdio>
#include <exception>
#include <filesystem>
#include <iostream>
#include <ostream>
#include <queue>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}

  // This won't balance a tree, but that is for another day
  TreeNode(std::vector<int> values) {
    val = values.at(0);
    left = nullptr;
    right = nullptr;
    for (int i = 1; i < values.size(); i++) {
      TreeNode *leaf = this;
      while (true) {
        // if less than current value - set leaf to left side...
        if (values.at(i) < leaf->val) {
          if (leaf->left == nullptr) {
            leaf->left = new TreeNode(values.at(i));
            break;
          } else {
            leaf = leaf->left;
          }
        } else {
          if (leaf->right == nullptr) {
            leaf->right = new TreeNode(values.at(i));
            break;
          } else {
            leaf = leaf->right;
          }
        }
      }
    }
  };

  void invert() {
    std::queue<TreeNode *> nodes;
    nodes.push(this);
    while (nodes.empty() == 0) {
      TreeNode *current = nodes.front();
      nodes.pop();
      // std::cout << current.val << std::endl;
      TreeNode *tmp = current->left;
      // left pointer is now right
      current->left = current->right;
      // right pointer is now left
      current->right = tmp;

      if (current->left != nullptr) {
        nodes.push(current->left);
      }
      if (current->right != nullptr) {
        nodes.push(current->right);
      }
    }
  };

  void print_tree() {
    std::queue<TreeNode *> nodes;
    nodes.push(this);

    // while not empty
    while (nodes.empty() == 0) {
      TreeNode *current = nodes.front();
      nodes.pop();
      std::cout << current->val << std::endl;

      if (current->left != nullptr) {
        nodes.push(current->left);
      }
      if (current->right != nullptr) {
        nodes.push(current->right);
      }
    }
  }
};

// Practice with inheriting constructors
class BinString : public std::string {
public:
  // inheritting constructors
  using std::string::string;

  // compiler sometimes doesn't pick the right one
  BinString(std::string &&str) : std::string(std::move(str)) {};

  std::vector<int> to_vec_int() const {
    std::vector<int> result;
    std::stringstream ss(*this);
    std::string elm;

    while (std::getline(ss, elm, ',')) {
      result.push_back(std::stoi(elm));
    };
    return result;
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

    BinString bin_str(std::move(line));
    std::vector<int> bin_vals = bin_str.to_vec_int();
    TreeNode *head = new TreeNode(bin_vals);
    head->print_tree();
    head->invert();
    head->print_tree();
    // Update String
    data.erase(0, pos + 1);
  }

  return 0;
}
