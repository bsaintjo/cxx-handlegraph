#include <iostream>
#include "rust_hash_graph/glue_hash_graph.hpp"
#include "rust_hash_graph/utils.hpp"
#include "lib.rs.h"

using namespace hgrs;
// using namespace glue_hash_graph;

// namespace glue_hash_graph
// {
//     class GlueHashGraph
//     {
//     public:
//         GlueHashGraph() : inner(new_hash_graph()) {};
//         ~GlueHashGraph() {};

//     private:
//         rust::cxxbridge1::Box<RustHashGraph> inner;
//     };
// }

using namespace glue_hash_graph;
using namespace utils;

int main(void)
{
    auto node_id = utils::to_rust_node_id(1);
    auto graph = new_hash_graph();
    graph->say_hello();
    std::cout << "Has node: " << graph->has_node(node_id) << std::endl;
    graph->print_occurences();
    std::cout << "Hello, world!" << std::endl;
    auto glue_graph = new GlueHashGraph();

    delete glue_graph;
    return 0;
}