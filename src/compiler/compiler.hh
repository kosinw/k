#include <iostream>
#include <optional>
#include "lexer.hh"

namespace kcc {
    class compiler
    {
    // TODO(kosi): Figure out how to do error handling properly.
    public:
        compiler();
        ~compiler();
        std::result<std::istream &> execute(const std::istream& input);

    private:
        kcc::lexer lexer;
    };
}