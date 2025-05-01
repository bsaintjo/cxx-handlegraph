#ifndef GLUE_HASH_GRAPH_HPP_INCLUDED
#define GLUE_HASH_GRAPH_HPP_INCLUDED

#include "handlegraph/handle_graph.hpp"
#include "lib.rs.h"

namespace glue_hash_graph {
using namespace std;
using namespace handlegraph;
using namespace hgrs;

class GlueHashGraph : public HandleGraph
// class GlueHashGraph
{
   public:
    GlueHashGraph();
    ~GlueHashGraph();

   private:
    rust::cxxbridge1::Box<RustHashGraph> inner;

    // Implementation of HandleGraph
   public:
    bool has_node(nid_t node_id) const override;
    handle_t get_handle(const nid_t& node_id, bool is_reverse = false) const override;
    nid_t get_id(const handle_t& handle) const override;
    bool get_is_reverse(const handle_t& handle) const override;
    handle_t flip(const handle_t& handle) const override;
    size_t get_length(const handle_t& handle) const override;
    std::string get_sequence(const handle_t& handle) const override;
    size_t get_node_count() const override;
    nid_t min_node_id() const override;
    nid_t max_node_id() const override;

   protected:
    bool follow_edges_impl(
        const handle_t& handle, bool go_left,
        const std::function<bool(const handle_t&)>& iteratee) const override;
    bool for_each_handle_impl(
        const std::function<bool(const handle_t&)>& iteratee,
        bool parallel = false) const override;
};
}  // namespace glue_hash_graph

#endif  // !RUST_HANDLE_GRAPH_IMPLEMENTATION_HPP_INCLUDED