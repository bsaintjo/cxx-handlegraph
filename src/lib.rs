use autocxx::prelude::*;

// include_cpp! {
//     #include "handlegraph/types.hpp"
//     #include "handlegraph/util.hpp"
//     #include "handlegraph/handle_graph.hpp"
//     #include "handlegraph/mutable_handle_graph.hpp"
//     // safety!(safe)
//     generate!("handlegraph::handle_t")
//     generate!("handlegraph::nid_t")
//     generate!("handlegraph::as_handle")
//     generate!("handlegraph::HandleGraph")
//     generate!("handlegraph::MutableHandleGraph")

// }
use handlegraph::{
    handle::NodeId,
    hashgraph::{HashGraph, Node},
    pathhandlegraph::{MutableGraphPaths, PathId},
};
use ref_cast::RefCast;

struct RustHashGraph(HashGraph);

#[derive(RefCast)]
#[repr(transparent)]
struct RustNode(Node);

fn new_node(sequence: &[u8]) -> Box<RustNode> {
    Box::new(RustNode(Node::new(sequence)))
}

impl RustHashGraph {
    fn print_occurences(&self) {
        self.0.print_occurrences();
    }

    fn get_node(&self, id: &ffi::RustNodeId) -> Result<*const RustNode, String> {
        self.0
            .get_node(&NodeId(id.inner))
            .map(|n| {
                let n = RustNode::ref_cast(n);
                let n: *const RustNode = n;
                n
            })
            .ok_or(String::from("Node not found"))
    }

    fn print_path(&self, path_id: &ffi::RustPathId) {
        self.0.print_path(&PathId(path_id.inner));
    }

    fn create_path(&mut self, name: &[u8], circular: bool) -> Result<ffi::RustPathId, String> {
        self.0
            .create_path(name, circular)
            .map(|path_id| ffi::RustPathId { inner: path_id.0 })
            .ok_or("Path creation failed".to_string())
    }
}

fn new_hash_graph() -> Box<RustHashGraph> {
    Box::new(RustHashGraph(HashGraph::new()))
}

#[cxx::bridge]
pub mod ffi {
    struct RustNodeId {
        inner: u64,
    }

    struct RustPathId {
        inner: u64,
    }
    extern "Rust" {
        type RustNode;
    }
    extern "Rust" {
        type RustHashGraph;
        fn new_hash_graph() -> Box<RustHashGraph>;
        fn print_occurences(&self);
        fn get_node(&self, node_id: &RustNodeId) -> Result<*const RustNode>;
        fn print_path(&self, path_id: &RustPathId);
        fn create_path(&mut self, name: &[u8], circular: bool) -> Result<RustPathId>;
    }
}

#[cfg(test)]
mod test {
    use std::pin::pin;

    // use crate::ffi::handlegraph::{HandleGraph, MutableHandleGraph};

    use super::*;

    // #[test]
    // fn test_handle_t() {
    //     let value = pin!(42);
    //     let handle = unsafe { ffi::handlegraph::as_handle(value) };
    // }

    // #[test]
    // fn test_handle_t() {
    //     let value: u64 = 42;
    //     let handle: &ffi::handle_t = ffi::as_handle(&value);
    //     // assert_eq!(handle, &value);
    // }
}
