#ifndef RUST_HANDLE_GRAPH_IMPLEMENTATION_HPP_INCLUDED
#define RUST_HANDLE_GRAPH_IMPLEMENTATION_HPP_INCLUDED


#include <handlegraph/handle_graph.hpp>

class GluedHashGraph : public HandleGraph, RustHashGraph {
    public:
        GluedHashGraph();
        ~GluedHashGraph();

        // Implementations of HandleGraph methods
        // bool has_node(nid_t node_id) const override;
        // handle_t get_handle(const nid_t& node_id, bool is_reverse = false) const override;
        // nid_t get_id(const handle_t& handle) const override;
        // bool get_is_reverse(const handle_t& handle) const override;
        // handle_t flip(const handle_t& handle) const override;
        // size_t get_length(const handle_t& handle) const override;
        // std::string get_sequence(const handle_t& handle) const override;
        // size_t get_node_count() const override;
        // nid_t min_node_id() const override;
        // nid_t max_node_id() const override;
}

#endif // !RUST_HANDLE_GRAPH_IMPLEMENTATION_HPP_INCLUDED