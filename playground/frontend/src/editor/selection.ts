import { Location } from "../compiler/client.js";
import { Cursor, editor, lines } from "./state.js";

export function selection_start(cursor: Cursor) {
    return Math.min(cursor.position, cursor.anchor);
}

export function selection_end(cursor: Cursor) {
    return Math.max(cursor.position, cursor.anchor);
}

export function has_selection(cursor: Cursor) {
    return selection_start(cursor) !== selection_end(cursor);
}

export function selected_text(cursor: Cursor) {
    return editor.text.slice(
        selection_start(cursor),
        selection_end(cursor)
    );
}

export function set_selection(position: number, anchor = position) {
    editor.cursors = [{ position, anchor }];
}

export function position_to_location(position: number): Location {
    const before = editor.text.slice(0, position);

    const line = before.split("\n").length - 1;

    const last_new_line = before.lastIndexOf("\n");

    const column =
        last_new_line === -1
            ? position
            : position - last_new_line - 1;

    return { line, column, pos: position };
}

export function location_to_position(line: number, column: number): number {
    return (
        lines()
            .slice(0, line)
            .reduce(
                (position, text) => position + text.length + 1,
                0
            ) + column
    );
}