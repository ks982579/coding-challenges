#include "mergelists.h"
#include <vector>
#include <string>

int main() {
    mergelists();

    std::vector<std::string> vec;
    vec.push_back("test_package");

    mergelists_print_vector(vec);
}
