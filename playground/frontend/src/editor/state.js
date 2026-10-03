
const STORAGE_KEY = "aero-playground-source";


export const SETTINGS = {
    tabSize: 4
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