import { editor } from "../editor/state.js";
import { handle_enter, handle_tab, select_all, handle_backspace, handle_arrow, add_cursor, insert_text } from "../editor/editor.js";
import { copy_selection, location_to_position, } from "../editor/selection.js";
import { render } from "./render.js";
import { get_location_from_mouse } from "./mouse.js";
import { analyze } from "../compiler/client.js";

export function setup_input(input: HTMLElement, viewport: HTMLElement) {
    const update = () => {
        analyze();
        render(viewport);
    };

    input.addEventListener("keydown", (event: KeyboardEvent) => {
        const { key, ctrlKey, metaKey, altKey, shiftKey } = event;
        const lower = event.key.toLowerCase();

        if (
            ctrlKey &&
            !shiftKey &&
            lower === "a"
        ) {
            event.preventDefault();
            select_all();
        } else if (key.startsWith("Arrow")) {
            event.preventDefault();
            handle_arrow(
                event.key,
                event.shiftKey
            );
        } else if (key === "Backspace") {
            event.preventDefault();
            handle_backspace();

        } else if (key === "Tab") {
            event.preventDefault();
            handle_tab();
        } else if (key === "Enter") {
            event.preventDefault();
            handle_enter();
        } else if (ctrlKey && lower == "c") {
            event.preventDefault();
            void copy_selection();
        }
        else if (
            key.length === 1 &&
            !ctrlKey &&
            !metaKey &&
            !altKey
        ) {
            event.preventDefault();
            insert_text(event.key);
        }
        update();
    });

    input.addEventListener("focus", () => {
        editor.focused = true;
        update();
    });

    input.addEventListener("blur", () => {
        editor.focused = false;
        update();
    });

    input.addEventListener("mousedown", (event: MouseEvent) => {
        const position = mouse_position(event, viewport);

        const cursor = event.altKey
            ? add_cursor(position)
            : { position, anchor: position };

        if (!event.altKey) {
            editor.cursors = [cursor];
        }

        input.focus({
            preventScroll: true,
        });

        update();

        const drag = (event: MouseEvent) => {
            const position = mouse_position(event, viewport);
            cursor.position = position;
            update();
        };

        const stop = () => {
            window.removeEventListener("mousemove", drag);
            window.removeEventListener("mouseup", stop);
        };

        window.addEventListener("mousemove", drag);
        window.addEventListener("mouseup", stop);
    });
}

function mouse_position(event: MouseEvent, viewport: HTMLElement) {
    const location = get_location_from_mouse(event, viewport);
    return location_to_position(location.line, location.column);
}