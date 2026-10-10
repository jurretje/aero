
const STORAGE_KEY = "aero-playground-source";

export type Settings = {
    tabSize: number;
    selectionMode: "text" | "token";
};

export type Cursor = {
    position: number;
    anchor: number;
}

export type Editor = {
    text: string;
    cursors: Cursor[];
    focused: boolean;
};

export const SETTINGS: Settings = {
    tabSize: 4,
    selectionMode: "text",
};

export const editor = {
    text: localStorage.getItem(STORAGE_KEY) ?? "",
    cursors: [{ position: 0, anchor: 0 }],
    focused: false,
};

export const lines = () => editor.text.split("\n");

export function save() {
    localStorage.setItem(STORAGE_KEY, editor.text);
}