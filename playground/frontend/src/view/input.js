import { editor } from "../editor/state.js";
import { selectAll, type, handleBackspace, handleArrow, addCursor, } from "../editor/editor.js";
import { handleEnter, handleTab, } from "../editor/editing.js";
import { locationToPosition, } from "../editor/selection.js";
import { render } from "./editor-view.js";
import { get_location_from_mouse } from "./mouse.js";

export function setup_input(input, viewport) {
    input.addEventListener("keydown", event => {
        const key = event.key.toLowerCase();

        if (
            event.ctrlKey &&
            !event.shiftKey &&
            key === "a"
        ) {
            event.preventDefault();

            selectAll();
            render(viewport);
            return;
        }

        if (event.key.startsWith("Arrow")) {
            event.preventDefault();

            handleArrow(
                event.key,
                event.shiftKey
            );

            render(viewport);
            return;
        }

        if (event.key === "Backspace") {
            event.preventDefault();

            handleBackspace();
            render(viewport);
            return;
        }

        if (event.key === "Tab") {
            event.preventDefault();

            handleTab();
            render(viewport);
            return;
        }

        if (event.key === "Enter") {
            event.preventDefault();

            handleEnter();
            render(viewport);
            return;
        }

        if (
            event.key.length === 1 &&
            !event.ctrlKey &&
            !event.metaKey &&
            !event.altKey
        ) {
            event.preventDefault();

            type(event.key);
            render(viewport);
        }
    });

    input.addEventListener("focus", () => {
        editor.focused = true;
        render(viewport);
    });

    input.addEventListener("blur", () => {
        editor.focused = false;
        render(viewport);
    });

    input.addEventListener("mousedown", event => {
        const location =
            get_location_from_mouse(
                event,
                viewport
            );

        const position =
            locationToPosition(
                location.line,
                location.column
            );

        if (event.altKey) {
            addCursor(position);
        } else {
            editor.cursors = [{
                position,
                anchor: position,
            }];
        }

        input.focus({
            preventScroll: true,
        });

        render(viewport);
    });
}