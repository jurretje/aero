import { editor, save, SETTINGS } from "./state.js";
import {
    selection_start,
    selection_end,
    has_selection,
    position_to_location,
} from "./selection.js";

export function insertText(text) {
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

    replaceRanges(edits);

    normalizeCursors();
    save();
}

export function handleEnter() {
    insertText("\n");
}

export function handleTab() {
    for (const cursor of editor.cursors) {
        const { column } =
            position_to_location(cursor.position);

        const count =
            SETTINGS.tabSize -
            (column % SETTINGS.tabSize);

        insertText(" ".repeat(count));
    }
}

export function replaceRanges(edits) {
    edits.sort((a, b) => b.start - a.start);

    for (const edit of edits) {
        editor.text =
            editor.text.slice(0, edit.start) +
            edit.text +
            editor.text.slice(edit.end);
    }
}

export function normalizeCursors() {
    const seen = new Set();

    editor.cursors = editor.cursors.filter(cursor => {
        if (seen.has(cursor.position)) {
            return false;
        }

        seen.add(cursor.position);
        return true;
    });
}