import { editor } from "../editor/state.js";
import {
    selection_start as selection_start,
    selection_end as selection_end,
    position_to_location as position_to_location,
    locationToPosition,
} from "../editor/selection.js";

export function render(viewport) {
    viewport.innerHTML = "";

    const lines = editor.text.split("\n");

    for (let i = 0; i < lines.length; i++) {
        viewport.appendChild(
            render_line(lines[i], i)
        );
    }
}

function render_line(text, lineNumber) {
    const line = document.createElement("div");
    line.className = "line-of-code";

    for (let i = 0; i < text.length; i++) {
        const char = document.createElement("span");

        char.textContent = text[i];

        const position =
            locationToPosition(lineNumber, i);

        for (const cursor of editor.cursors) {
            const start = selection_start(cursor);
            const end = selection_end(cursor);

            if (
                position >= start &&
                position < end
            ) {
                char.classList.add("selected");
            }
        }

        line.appendChild(char);
    }

    for (const cursor of editor.cursors) {
        const location =
            position_to_location(cursor.position);

        if (location.line !== lineNumber) {
            continue;
        }

        const caret =
            document.createElement("span");

        caret.className = "cursor";
        caret.style.left =
            `${location.column}ch`;

        line.appendChild(caret);
    }

    return line;
}