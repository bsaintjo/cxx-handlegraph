// use autocxx::prelude::*;

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
    handle::{Handle, NodeId}, handlegraph::{HandleGraph, IntoHandles, IntoSequences}, hashgraph::{HashGraph, Node}, mutablehandlegraph::AdditiveHandleGraph, pathhandlegraph::{MutableGraphPaths, PathId}
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

    fn say_hello(&self) {
        println!("Hello from Rust!");
    }

    fn has_node(&self, id: &ffi::RustNodeId) -> bool {
        self.0.has_node(NodeId(id.inner))
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

    fn min_node_id(&self) -> ffi::RustNodeId {
        ffi::RustNodeId {
            inner: self.0.min_node_id().0,
        }
    }

    fn max_node_id(&self) -> ffi::RustNodeId {
        ffi::RustNodeId {
            inner: self.0.max_node_id().0,
        }
    }

    fn get_node_count(&self) -> usize {
        self.0.node_count()
    }

    fn append_handle(&mut self, sequence: &[u8]) -> ffi::RustHandle {
        ffi::RustHandle { inner: self.0.append_handle(sequence).0 }
    }

    fn get_handle(&self, node_id: &ffi::RustNodeId, is_reverse: bool) -> ffi::RustHandle {
        let handle = Handle::pack(node_id.inner, is_reverse);
        ffi::RustHandle{ inner: handle.0 }
        // ffi::RustHandle(self.0.get_handle(NodeId(node_id.inner), is_reverse))
    }

    fn get_sequence(&self, handle: &ffi::RustHandle) -> String {
        let handle = Handle(handle.inner);
        String::from_utf8(self.0.sequence_vec(handle)).unwrap()
    }

    fn get_is_reverse(&self, handle: &ffi::RustHandle) -> bool {
        let handle = Handle(handle.inner);
        handle.is_reverse()
    }

    fn flip(&self, handle: &ffi::RustHandle) -> ffi::RustHandle {
        let handle = Handle(handle.inner);
        ffi::RustHandle { inner: handle.flip().0 }
    }
    fn get_id(&self, handle: &ffi::RustHandle) -> ffi::RustNodeId {
        let handle = Handle(handle.inner);
        ffi::RustNodeId { inner: handle.id().0 }
    }

    fn get_length(&self, handle: &ffi::RustHandle) -> usize {
        let handle = Handle(handle.inner);
        self.0.node_len(handle)
    }
}

fn new_hash_graph() -> Box<RustHashGraph> {
    Box::new(RustHashGraph(HashGraph::new()))
}

#[cxx::bridge(namespace="hgrs")]
pub mod ffi {
    struct RustNodeId {
        inner: u64,
    }

    struct RustPathId {
        inner: u64,
    }

    struct RustHandle {
        inner: u64,
    }

    #[namespace="handlegraph"]
    extern "C++" {
        include!("handlegraph/types.hpp");
        // include!("handlegraph/util.hpp");
    //     include!("handlegraph/handle_graph.hpp");
    //     include!("handlegraph/mutable_handle_graph.hpp");
    //     include!("handlegraph/path_handle_graph.hpp");

        type handle_t;
    }

    extern "Rust" {
        type RustNode;
    }
    extern "Rust" {
        type RustHashGraph;
        fn new_hash_graph() -> Box<RustHashGraph>;
        fn print_occurences(&self);
        fn say_hello(&self);
        fn has_node(&self, node_id: &RustNodeId) -> bool;
        fn get_node(&self, node_id: &RustNodeId) -> Result<*const RustNode>;
        fn print_path(&self, path_id: &RustPathId);
        fn create_path(&mut self, name: &[u8], circular: bool) -> Result<RustPathId>;
        fn min_node_id(&self) -> RustNodeId;
        fn max_node_id(&self) -> RustNodeId;
        fn get_node_count(&self) -> usize;
        fn get_handle(&self, node_id: &RustNodeId, is_reverse: bool) -> RustHandle;
        fn get_sequence(&self, handle: &RustHandle) -> String;
        fn get_is_reverse(&self, handle: &RustHandle) -> bool;
        fn flip(&self, handle: &RustHandle) -> RustHandle;
        fn get_id(&self, handle: &RustHandle) -> RustNodeId;
        fn get_length(&self, handle: &RustHandle) -> usize;
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
