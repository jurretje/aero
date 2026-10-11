import { handle_arrow, handle_backspace, handle_enter, handle_tab, insert_text, select_all } from "../editor/editor.js";

export type InputCommand = {
    matches: (event: KeyboardEvent) => boolean;
    execute: (event: KeyboardEvent) => void | Promise<void>;
};

export const input_commands: InputCommand[] = [
    {
        matches: ({ ctrlKey, metaKey, shiftKey, key }) =>
            (ctrlKey || metaKey) && !shiftKey && key.toLowerCase() === "a",
        execute: () => select_all()
    },
    {
        matches: ({ key }) => key.startsWith("Arrow"),
        execute: ({ key, shiftKey }) => handle_arrow(key, shiftKey),
    },
    {
        matches: ({ key }) => key === "Backspace",
        execute: () => handle_backspace(),
    },
    {
        matches: ({ key }) => key === "Tab",
        execute: () => handle_tab(),
    },
    {
        matches: ({ key }) => key === "Enter",
        execute: () => handle_enter(),
    },
    {
        matches: ({ key, ctrlKey, metaKey, altKey }) => key.length === 1 && !ctrlKey && !metaKey && !altKey,
        execute: ({ key }) => {
            insert_text(key)
        }
    }
];