#include "rust_hash_graph/glue_hash_graph.hpp"

#include "handlegraph/util.hpp"
#include "lib.rs.h"
#include "rust_hash_graph/utils.hpp"

namespace glue_hash_graph {
using namespace handlegraph;
using namespace hgrs;
using namespace glue_hash_graph;
using namespace utils;

GlueHashGraph::GlueHashGraph() : inner(hgrs::new_hash_graph()) {}

GlueHashGraph::~GlueHashGraph() {}

bool GlueHashGraph::has_node(nid_t node_id) const {
    RustNodeId node_id_rust = utils::to_rust_node_id(node_id);
    return inner->has_node(node_id_rust);
}

nid_t GlueHashGraph::min_node_id() const {
    return utils::from_rust_node_id(inner->min_node_id());
}

nid_t GlueHashGraph::max_node_id() const {
    return utils::from_rust_node_id(inner->max_node_id());
}

bool GlueHashGraph::follow_edges_impl(
    const handle_t &handle, bool go_left,
    const std::function<bool(const handle_t &)> &iteratee) const {
    return false;
}

bool GlueHashGraph::for_each_handle_impl(
    const std::function<bool(const handle_t &)> &iteratee,
    bool parallel) const {
    return false;
}

size_t GlueHashGraph::get_node_count() const { return inner->get_node_count(); }

handle_t GlueHashGraph::get_handle(const nid_t &node_id,
                                   bool is_reverse) const {
    RustNodeId node_id_rust = utils::to_rust_node_id(node_id);
    RustHandle res = inner->get_handle(node_id_rust, is_reverse);
    return utils::from_rust_handle(res);
}

nid_t GlueHashGraph::get_id(const handle_t &handle) const {
    auto handle_rust = utils::to_rust_handle(handle);
    auto node_id_rust = inner->get_id(handle_rust);
    return from_rust_node_id(node_id_rust);
}
bool GlueHashGraph::get_is_reverse(const handle_t &handle) const {
    auto handle_rust = utils::to_rust_handle(handle);
    return inner->get_is_reverse(handle_rust);
}

handle_t GlueHashGraph::flip(const handle_t &handle) const {
    auto handle_rust = utils::to_rust_handle(handle);
    auto flipped_handle = inner->flip(handle_rust);
    return utils::from_rust_handle(flipped_handle);
}

size_t GlueHashGraph::get_length(const handle_t &handle) const {
    auto handle_rust = utils::to_rust_handle(handle);
    auto length = inner->get_length(handle_rust);
    return length;
}
std::string GlueHashGraph::get_sequence(const handle_t &handle) const {
    RustHandle handle_rust = utils::to_rust_handle(handle);
    auto seq = inner->get_sequence(handle_rust);
    return std::string(seq);
}
}  // namespace glue_hash_graph