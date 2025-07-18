#include <xmminitrin.h>
#include <stdio.h>
#include <istream>
#include <ostream>
#include <bitset>

class Manipulator {


};

class FileManipulator: private Manipulator {
    std::istream& i;
    std::ostream& o;
    std::bitset<32> config;

    explicit FileManipulator(std::istream& i, std::ostream& o, std::bitset<32> config): i(i), o(o), config(config) {}

    void read_file(std::istream &i) {

    }
};

int main(int argc, char** argv) {
    __m256 a = 1;
    __m256 b = 2;
    __m256 v = __mm256_add_ps(a, b);
    printf("%d", v);    
}