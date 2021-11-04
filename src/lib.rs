#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
mod app;
mod event;
mod graph;
mod nodes;
mod state;
mod update;
use app::create_window;
use graph::NodeGraph;
use num_traits::FromPrimitive;
use state::State;
use std::cell::RefCell;

// thread_local! {
//     static NODE_GRAPH: RefCell<NodeGraph> = RefCell::new(NodeGraph::new());
// }

#[no_mangle]
pub extern "C" fn ops_create_window(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let string = std::str::from_utf8(buf).unwrap();
    create_window(string).unwrap();
}

// #[no_mangle]
// pub extern "C" fn ops_create_element() {
//     NODE_GRAPH.with(|nodes| {
//         let mut nodesref = nodes.borrow_mut();

//         nodesref.create_proto()
//     });
// }

// #[no_mangle]
// pub extern "C" fn ops_add_child_element(id: u64, el: u32) -> u64 {
//     NODE_GRAPH.with(|nodes| {
//         let mut nodesref = nodes.borrow_mut();

//         let element = FromPrimitive::from_u32(el).unwrap();
//         nodesref.add_child(id, element)
//     })
// }

// #[no_mangle]
// pub extern "C" fn ops_set_state(id: u64, state: u32, ptr: *const u8, len: usize) {
//     NODE_GRAPH.with(|nodes| {
//         let mut nodesref = nodes.borrow_mut();

//         let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
//         let state = FromPrimitive::from_u32(state).unwrap();
//         let node = nodesref.lookup(id);
//         node.set_state(State::from(state, buf))
//     });
// }

#[no_mangle]
pub extern "C" fn destroy_element() {}
