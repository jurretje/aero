import { setup_input } from "./view/input.js";
import { render } from "./view/render.js";
import { analyze } from "./compiler/client.js";


async function main() {

    const run_button = document.getElementById("run-button")!;
    const viewport = document.getElementById("viewport")!;
    const input = document.getElementById("input")!;

    run_button.addEventListener(
        "click",
        analyze
    );

    setup_input(input, viewport);
    await analyze();
    render(viewport);
}

main().catch(console.error);