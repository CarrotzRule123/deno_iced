#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use std::future::Future;
use std::thread;

mod app;
mod event;
mod graph;
mod nodes;
mod state;
mod update;
use app::create_window;
use graph::NodeGraph;
use state::State;
use update::{Update, UpdateFuture};

lazy_static! {
    pub static ref RESOURCES: Resources = Resources::new();
}

#[no_mangle]
pub extern "C" fn ops_create_window(id: u64, ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let string = std::str::from_utf8(buf).unwrap();
    let handle = thread::spawn(move || {
        create_window(id, string).unwrap();
    });
    handle.join().unwrap();
}

// #[no_mangle]
// pub extern "C" fn ops_create_element() {
//     // NODE_GRAPH.with(|nodes| {
//     //     let mut nodesref = nodes.borrow_mut();

//     //     nodesref.create_proto()
//     // });
// }

#[no_mangle]
pub extern "C" fn ops_add_child_element(el: u32, child: u64, parent: u64) {
    println!("aaa");
    let element = FromPrimitive::from_u32(el).unwrap();
    RESOURCES
        .update
        .update(Update::AddChild(element, child, parent));
}

// #[no_mangle]
// pub extern "C" fn ops_set_state(id: u64, state: u32, ptr: *const u8, len: usize) {
//     let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
//     let state = FromPrimitive::from_u32(state).unwrap();
    
//     RESOURCES
//         .update
//         .update(Update::SetState(id, child_el, state));
// }

#[no_mangle]
pub extern "C" fn destroy_element() {}

pub struct Resources {
    update: UpdateFuture,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            update: UpdateFuture::new(),
        }
    }
}
