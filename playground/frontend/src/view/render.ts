import { editor, SETTINGS } from "../editor/state.js";
import { selection_start, selection_end, position_to_location, location_to_position } from "../editor/selection.js";

import { compile_result, StyledToken } from "../compiler/client.js";

type Selection = {
    start: number,
    end: number,
};

function get_selections(): Selection[] {
    return editor.cursors.map(cursor => ({
        start: selection_start(cursor),
        end: selection_end(cursor),
    })).filter(({ start, end }) => start !== end);
}

function is_selected(start: number, end: number, selections: Selection[]): boolean {
    return selections.some(selection => selection.start <= start && end <= selection.end);
}

export function render(viewport: HTMLElement) {
    viewport.innerHTML = "";

    const lines = editor.text.split("\n");

    for (let line_number = 0; line_number < lines.length; line_number++) {
        viewport.appendChild(
            render_line((lines[line_number] ?? ""), line_number)
        );
    }
}

function render_line(text: string, line_number: number) {
    const line = document.createElement("div");
    line.className = "line-of-code";

    const line_start = location_to_position(line_number, 0);
    const line_end = line_start + text.length;

    const tokens = (compile_result?.tokens ?? []).filter(token =>
        token.span.start.pos < line_end &&
        token.span.end.pos > line_start
    );

    const selections = get_selections();

    const boundaries = new Set<number>([line_start, line_end]);

    for (const token of tokens) {
        boundaries.add(Math.max(token.span.start.pos, line_start));
        boundaries.add(Math.min(token.span.end.pos, line_end));
    }

    for (const selection of selections) {
        if (selection.start > line_start && selection.start < line_end) {
            boundaries.add(selection.start);
        }

        if (selection.end > line_start && selection.end < line_end) {
            boundaries.add(selection.end);
        }
    }

    const sorted = [...boundaries]
        .filter(pos => pos >= line_start && pos <= line_end)
        .sort((a, b) => a - b);

    const append = (
        start: number,
        end: number,
        token: StyledToken | null
    ) => {
        if (end <= start) return;

        if (!token) {
            const span = document.createElement("span");
            span.textContent = editor.text.slice(start, end);

            if (
                SETTINGS.selection_mode !== "token" &&
                is_selected(start, end, selections)
            ) {
                span.classList.add("selected");
            }

            line.appendChild(span);
            return;
        }

        const token_start = token.span.start.pos;
        const token_end = token.span.end.pos;

        const parent = document.createElement("span");
        parent.classList.add(`${token.style.toLowerCase()}-token`);

        const cursor_on_token = editor.cursors.some(cursor =>
            cursor.position >= token_start &&
            cursor.position < token_end
        );

        if (cursor_on_token) {
            parent.classList.add("token-at-cursor");
        }

        if (
            SETTINGS.selection_mode === "token" &&
            is_selected(token_start, token_end, selections)
        ) {
            parent.classList.add("selected-token");
        }

        const boundaries = new Set<number>([start, end]);

        if (SETTINGS.selection_mode !== "token") {
            for (const selection of selections) {
                if (selection.start > start && selection.start < end) {
                    boundaries.add(selection.start);
                }

                if (selection.end > start && selection.end < end) {
                    boundaries.add(selection.end);
                }
            }
        }

        const sorted = [...boundaries].sort((a, b) => a - b);

        for (let i = 0; i < sorted.length - 1; i++) {
            const segment_start = sorted[i]!;
            const segment_end = sorted[i + 1]!;

            const child = document.createElement("span");
            child.textContent = editor.text.slice(segment_start, segment_end);

            if (
                SETTINGS.selection_mode !== "token" &&
                is_selected(segment_start, segment_end, selections)
            ) {
                child.classList.add("selected");
            }

            parent.appendChild(child);
        }

        line.appendChild(parent);
    };

    for (let i = 0; i < sorted.length - 1; i++) {
        const start = sorted[i]!;
        const end = sorted[i + 1]!;

        const token = tokens.find(token =>
            token.span.start.pos <= start &&
            token.span.end.pos >= end
        ) ?? null;

        append(start, end, token);
    }

    append_cursors();
    return line;

    function append_cursors() {
        for (const cursor of editor.cursors) {
            const location = position_to_location(cursor.position);

            if (location.line !== line_number) continue;

            const caret = document.createElement("span");
            caret.className = "cursor";
            caret.style.left = `${location.column}ch`;

            line.appendChild(caret);
        }
    }
}