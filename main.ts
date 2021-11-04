import { Plug } from "./deps.ts";

const options: Plug.Options = {
    name: "test_lib",
    urls: {
        darwin: `target/debug/deno_iced.dylib`,
        windows: `target/debug/deno_iced.dll`,
        linux: `target/debug/deno_iced.so`,
    },
};

export const library = await Plug.prepare(options, {
    ops_create_window: {
        parameters: ["buffer", "usize"],
        result: "void",
    },
    // ops_add_child_element: {
    //     parameters: ["u64", "u32"],
    //     result: "u64",
    // },
    // ops_set_state: {
    //     parameters: ["u32", "u32", "buffer", "usize"],
    //     result: "void",
    // },
});

export type Library = typeof library