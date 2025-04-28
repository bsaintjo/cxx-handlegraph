// #include "cxxbridge/cxx-handlegraph/src/lib.rs.h"
#include "lib.rs.h"
#include "handlegraph/handle_graph.hpp"
#include "handlegraph/util.hpp"

namespace utils
{
    using namespace handlegraph;
    using namespace hgrs;

    RustNodeId to_rust_node_id(nid_t node_t)
    {
        RustNodeId node_id;
        node_id.inner = static_cast<uint64_t>(node_t);
        return node_id;
    }

    nid_t from_rust_node_id(RustNodeId rust_node_id)
    {
        return static_cast<nid_t>(rust_node_id.inner);
    }

    RustHandle to_rust_handle(handle_t handle) {
        RustHandle rhandle;
        rhandle.inner = handlegraph::as_integer(handle);
        return rhandle;
    }

    handle_t from_rust_handle(RustHandle rust_handle) {
        return handlegraph::as_handle(rust_handle.inner);
    }
}