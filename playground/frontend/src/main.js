import { setup_input } from "./view/input.js";
import { render } from "./view/render.js";
import { editor } from "./editor/state.js";
import { request_compile } from "./compiler/client.js";

const runButton = document.getElementById("run-button");
const viewport = document.getElementById("viewport");
const input = document.getElementById("input");

export let compile_result = null;

async function compile() {
    try {
        compile_result = await request_compile(
            editor.text,
            editor.cursors
        );
    } catch (error) {
        console.error(error);
    }
}

runButton.addEventListener(
    "click",
    compile
);

setup_input(input, viewport);
await compile();
render(viewport);