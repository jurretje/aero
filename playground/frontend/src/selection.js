import { editor, save } from "./state.js";

export const selectionStart = (cursor) => Math.min(cursor.position, cursor.anchor);
export const selectionEnd = (cursor) => Math.max(cursor.position, cursor.anchor);
export const selected = (cursor) => selectionStart(cursor) !== selectionEnd(cursor);


export function setSelection(position, anchor = position) {
    editor.cursors = [{ position, anchor }];
}

export function selectedText(cursor) {
    return editor.text.slice(selectionStart(cursor), selectionEnd(cursor));
}

export function positionToLocation(position) {
    const before = editor.text.slice(0, position);
    const line = before.split("\n").length - 1;
    const lastNewline = before.lastIndexOf("\n");
    const column = lastNewline === -1 ? position : position - lastNewline - 1;
    return { line, column };
}

export function locationToPosition(line, column) {
    return (
        lines()
            .slice(0, line)
            .reduce((position, text) => position + text.length + 1, 0) + column
    );
}
