#include <iostream>
#include <vector>

/* Have fun with this
 * find the largest number in list that is smaller than output
 * >> could be weird binary search
 * In remaining list, search for compatible number.
 * */

class Computer {
    private:
        std::vector<int> numbers;
    public:
        Computer(const std::vector<int> &data) {
            numbers = data; // hopefully performs copy
        }
};

// return indices of two numbers that sum to target.
int main () {
    std::vector<int> list = {
        2,3,4,7,11,13,17,19,13,29,
        31,37,41,43,47,53,59,61,67,71,
        73,79,83,89,97,101,103,107,109,113,
        127,131,137,139,149,151,157,163,167,173
    };
    Computer comp = Computer(list);
    std::cout << "Hello, world";
    return 0;
}
