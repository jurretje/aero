import { editor } from "../editor/state.js";
import {
    selection_start as selection_start,
    selection_end as selection_end,
    position_to_location as position_to_location,
    location_to_position,
} from "../editor/selection.js";

import { compile_result } from "../main.js";

export function render(viewport) {
    viewport.innerHTML = "";

    const lines = editor.text.split("\n");

    for (let line_number = 0; line_number < lines.length; line_number++) {
        viewport.appendChild(
            render_line(lines[line_number], line_number)
        );
    }
}

function render_line(text, line_number) {
    const line = document.createElement("div");
    line.className = "line-of-code";

    const line_start = location_to_position(line_number, 0);
    const line_end = line_start + text.length;

    const append = (content, column, token = null) => {
        const span = document.createElement("span");
        span.className = token ? `token ${token.style.toLowerCase()}-token` : "token";
        span.textContent = content;

        if (is_selected(column, column + content.length)) {
            span.classList.add("selected");
        }

        line.appendChild(span);
    };

    const is_selected = (start, end) => {
        return editor.cursors.some(cursor =>
            (line_start + start) < selection_end(cursor) &&
            (line_start + end) > selection_start(cursor)
        );
    };

    if (!compile_result) {
        append(text, 0);
        return line;
    }

    const tokens = compile_result.tokens.filter(token => {
        const start = token.span.start.pos;
        const end = token.span.end.pos;

        return start < line_end && end > line_start;
    });

    let column = 0;
    for (const token of tokens) {
        const token_start = token.span.start.pos;
        const token_end = token.span.end.pos;

        const start = Math.max(token_start - line_start, 0);
        const end = Math.min(token_end - line_start, text.length);

        if (start > column) {
            append(text.slice(column, start), column);
        }

        if (end > start) {
            append(text.slice(start, end), start, token)
        }

        column = end;
    }

    if (column < text.length) {
        append(text.slice(column), column);
    }

    append_cursors();
    return line;

    function append_cursors() {
        for (const cursor of editor.cursors) {
            const location = position_to_location(cursor);

            if (location.line !== line_number) {
                continue;
            }

            const caret = document.createElement("span");
            caret.className = "cursor";
            caret.style.left = `${location.column}ch`;

            line.appendChild(caret);
        }
    }
}
