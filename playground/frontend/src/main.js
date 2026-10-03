import { setup_input } from "./view/input.js";
import { render } from "./view/editor-view.js";
import { editor } from "./editor/state.js";
import { request_compile } from "./compiler/client.js";

const runButton = document.getElementById("run-button");

const viewport = document.getElementById("viewport");

const input = document.getElementById("input");

async function compile() {
    try {
        const result = await request_compile(
            editor.text,
            editor.cursors
        );

        console.log(result);
    } catch (error) {
        console.error(error);
    }
}

runButton.addEventListener(
    "click",
    compile
);

setup_input(input, viewport);
render(viewport);