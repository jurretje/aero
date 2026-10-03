import { editor } from "./state.js";
import { set_selection } from "./selection.js";
import {
    insertText,
    backspace,
} from "./editing.js";
import { moveCursors } from "./movement.js";

export function selectAll() {
    set_selection(
        editor.text.length,
        0
    );
}

export function type(text) {
    insertText(text);
}

export function handleBackspace() {
    backspace();
}

export function handleArrow(key, selecting) {
    moveCursors(key, selecting);
}

export function addCursor(position) {
    if (
        editor.cursors.some(
            cursor => cursor.position === position
        )
    ) {
        return;
    }

    editor.cursors.push({
        position,
        anchor: position,
    });
}