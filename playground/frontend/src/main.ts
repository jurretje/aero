import { setup_input } from "./input/input.js";
import { render } from "./view/render.js";
import { analyze, init_request } from "./compiler/client.js";


async function main() {
    const run_button = document.getElementById("run-button")!;
    const viewport = document.getElementById("viewport")!;
    const input = document.getElementById("input")!;

    run_button.addEventListener(
        "click",
        analyze
    );

    setup_input(input, viewport);
    await init_request();
    render(viewport);
}

main().catch(console.error);