export const library = Deno.dlopen(`target/debug/deno_iced.dll`, {
    ops_create_window: {
        parameters: ["u64", "buffer", "usize"],
        result: "void",
    },
    ops_add_child_element: {
        parameters: ["u32", "u64", "u64"],
        result: "void",
    },
    // ops_set_state: {
    //     parameters: ["u32", "u32", "buffer", "usize"],
    //     result: "void",
    // },
});