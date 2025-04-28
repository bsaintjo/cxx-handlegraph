#include <iostream>

#include "lib.rs.h"
#include "handlegraph/types.hpp"
#include "rust_hash_graph/glue_hash_graph.hpp"

int main(void) {
    auto graph = new glue_hash_graph::GlueHashGraph();
    delete graph;
    std::cout << "Hello, world!" << std::endl;
}