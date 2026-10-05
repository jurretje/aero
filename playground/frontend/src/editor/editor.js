import { editor, save, SETTINGS } from "./state.js";
import {
    selection_start,
    selection_end,
    set_selection,
    position_to_location,
} from "./selection.js";

import { move_cursors } from "./movement.js";

export function insert_text(text) {
    const cursors = [...editor.cursors]
        .sort(
            (a, b) =>
                selection_start(b) - selection_start(a)
        );

    for (const cursor of cursors) {
        const start = selection_start(cursor);
        const end = selection_end(cursor);

        editor.text =
            editor.text.slice(0, start) +
            text +
            editor.text.slice(end);

        const position = start + text.length;

        cursor.position = position;
        cursor.anchor = position;
    }

    save();
}

export function backspace() {
    const edits = editor.cursors
        .map(cursor => {
            const start = selection_start(cursor);
            const end = selection_end(cursor);

            if (start !== end) {
                cursor.position = start;
                cursor.anchor = start;

                return {
                    start,
                    end,
                    text: "",
                };
            }

            if (cursor.position === 0) {
                return null;
            }

            const position = cursor.position;

            cursor.position--;
            cursor.anchor--;

            return {
                start: position - 1,
                end: position,
                text: "",
            };
        })
        .filter(Boolean);

    replace_ranges(edits);

    normalize_cursors();
    save();
}

export function handle_enter() {
    insert_text("\n");
}

export function handle_tab() {
    for (const cursor of editor.cursors) {
        const { column } =
            position_to_location(cursor.position);

        const count =
            SETTINGS.tabSize -
            (column % SETTINGS.tabSize);

        insert_text(" ".repeat(count));
    }
}

export function replace_ranges(edits) {
    edits.sort((a, b) => b.start - a.start);

    for (const edit of edits) {
        editor.text =
            editor.text.slice(0, edit.start) +
            edit.text +
            editor.text.slice(edit.end);
    }
}

export function normalize_cursors() {
    const seen = new Set();

    editor.cursors = editor.cursors.filter(cursor => {
        if (seen.has(cursor.position)) {
            return false;
        }

        seen.add(cursor.position);
        return true;
    });
}

export function select_all() {
    set_selection(
        editor.text.length,
        0
    );
}

export function handle_backspace() {
    backspace();
}

export function handle_arrow(key, selecting) {
    move_cursors(key, selecting);
}

export function add_cursor(position) {
    if (editor.cursors.some(cursor => cursor.position === position)) {
        return;
    }

    const cursor = {
        position,
        anchor: position,
    };

    editor.cursors.push(cursor);
    return cursor;
}