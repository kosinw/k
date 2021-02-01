#include <iostream>

namespace kcc {
    class lexer {
    public:
        lexer();
        ~lexer();
    public:
        bool tokenize(std::istream& input_stream);
    };
}