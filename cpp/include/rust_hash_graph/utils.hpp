#ifndef GLUE_UTILS_HPP_INCLUDED
#define GLUE_UTILS_HPP_INCLUDED

#include "lib.rs.h"
#include "handlegraph/handle_graph.hpp"

namespace utils
{
    using namespace handlegraph;
    using namespace hgrs;

    RustNodeId to_rust_node_id(nid_t node_t);
    nid_t from_rust_node_id(RustNodeId rust_node_id);

    RustHandle to_rust_handle(handle_t handle);
    handle_t from_rust_handle(RustHandle rust_handle);
}
#endif // !GLUE_UTILS_HPP_INCLUDED