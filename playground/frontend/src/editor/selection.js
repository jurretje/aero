import { editor, lines } from "./state.js";

export function selection_start(cursor) {
    return Math.min(cursor.position, cursor.anchor);
}

export function selection_end(cursor) {
    return Math.max(cursor.position, cursor.anchor);
}

export function has_selection(cursor) {
    return selection_start(cursor) !== selection_end(cursor);
}

export function selected_text(cursor) {
    return editor.text.slice(
        selection_start(cursor),
        selection_end(cursor)
    );
}

export function set_selection(position, anchor = position) {
    editor.cursors = [{ position, anchor }];
}

export async function copy_selection() {
    const selections = editor.cursors.filter(cursor => cursor.position !== cursor.anchor).map(selected_text);
    if (selections.length === 0) return;

    await navigator.clipboard.writeText(selections.join("\n"));
}

export function position_to_location(position) {
    const before = editor.text.slice(0, position);

    const line = before.split("\n").length - 1;

    const lastNewline = before.lastIndexOf("\n");

    const column =
        lastNewline === -1
            ? position
            : position - lastNewline - 1;

    return { line, column };
}

export function location_to_position(line, column) {
    return (
        lines()
            .slice(0, line)
            .reduce(
                (position, text) => position + text.length + 1,
                0
            ) + column
    );
}