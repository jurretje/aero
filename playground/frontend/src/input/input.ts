import { editor } from "../editor/state.js";
import { location_to_position, selected_text, } from "../editor/selection.js";
import { render } from "../view/render.js";
import { get_location_from_mouse } from "../view/mouse.js";
import { analyze } from "../compiler/client.js";
import { input_commands } from "./command_inputs.js";
import { add_cursor, insert_text } from "../editor/editor.js";

export async function dispatch_input(event: KeyboardEvent): Promise<boolean> {
    const command = input_commands.find((command) => command.matches(event));

    if (!command) return false;

    await command.execute(event);
    return true;
}

export async function setup_input(input: HTMLElement, viewport: HTMLElement) {
    const update = async () => {
        await analyze();
        render(viewport);
    };

    input.addEventListener("keydown", async (event: KeyboardEvent) => {
        const handled = await dispatch_input(event);
         
        if (handled) {
            await update();
            event.preventDefault();
        }
    });

    input.addEventListener("focus", () => {
        editor.focused = true;
        update();
    });

    input.addEventListener("blur", () => {
        editor.focused = false;
        update();
    });

    input.addEventListener("paste", async (event: ClipboardEvent) => {
        const text = event.clipboardData?.getData("text/plain");
        if (text === undefined) return;
        event.preventDefault();
        insert_text(text);
        await update();
    });

    input.addEventListener("copy", async (event: ClipboardEvent) => {
        const selections = editor.cursors.filter(cursor => cursor.position !== cursor.anchor).map(selected_text);

        if (selections.length === 0) return;

        event.clipboardData?.setData("text/plain", selections.join("\n"));

        event.preventDefault();
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