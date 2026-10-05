import { editor } from "./state.js";
import {
    position_to_location,
    location_to_position,
} from "./selection.js";
import { normalize_cursors } from "./editor.js";

export function move_cursors(key, selecting) {
    for (const cursor of editor.cursors) {
        let position = cursor.position;

        if (key === "ArrowLeft") {
            position = Math.max(0, position - 1);
        }

        if (key === "ArrowRight") {
            position = Math.min(
                editor.text.length,
                position + 1
            );
        }

        if (key === "ArrowUp") {
            position = move_vertical(position, -1);
        }

        if (key === "ArrowDown") {
            position = move_vertical(position, 1);
        }

        cursor.position = position;

        if (!selecting) {
            cursor.anchor = position;
        }
    }

    normalize_cursors();
}

function move_vertical(position, direction) {
    const { line, column } =
        position_to_location(position);

    const lines = editor.text.split("\n");

    const newLine = line + direction;

    if (
        newLine < 0 ||
        newLine >= lines.length
    ) {
        return position;
    }

    const newColumn = Math.min(
        column,
        lines[newLine].length
    );

    return location_to_position(
        newLine,
        newColumn
    );
}