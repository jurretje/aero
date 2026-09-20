const viewport = document.getElementById("viewport");
const input = document.getElementById("input");


const STORAGE_KEY = "aero-playground-source";
const SETTINGS = {
    tabSize: 4
};

const editor = {
    text: localStorage.getItem(STORAGE_KEY) ?? "",
    cursors: [{ line: 0, column: 0, position: 0 }],
    focused: false,
}



function getLocationFromMouse(event) {
    const rect = viewport.getBoundingClientRect();

    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const lines = editor.text.split("\n");

    const lineHeight = parseFloat(getComputedStyle(viewport).lineHeight); // todo

    let line = Math.floor(y / lineHeight);

    line = Math.max(0, Math.min(line, lines.length - 1));

    const text = lines[line];

    const style = getComputedStyle(viewport);
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    context.font = `${style.fontSize} ${style.fontFamily}`;

    const charWidth = context.measureText("M").width;
    let column = Math.round(x / charWidth);

    column = Math.max(0, Math.min(column, text.length));

    return {
        line,
        column,
    };
}


function saveEditor() {
    localStorage.setItem(STORAGE_KEY, editor.text);
}

function render() {
    viewport.innerHTML = "";
    const lines = editor.text.split("\n");
    for (let i = 0; i < lines.length; i++) {
        const line = document.createElement("div");
        line.className = "line-of-code";

        const text = document.createElement("span");
        text.textContent = lines[i];

        line.appendChild(text);

        if (editor.focused) {
            for (const cursor of editor.cursors) {
                if (cursor.line === i) {
                    const caret = document.createElement("span");
                    caret.className = "cursor";
                    caret.style.left = `${cursor.column}ch`;
                    line.appendChild(caret);
                }
            }
        }


        viewport.appendChild(line);
    }
}

function handleTab() {
    const { column } = editor.cursors[0];

    const count = SETTINGS.tabSize - (column % SETTINGS.tabSize);

    insertText(" ".repeat(count));
}

function handleBackspace() {
    const cursor = editor.cursors[0];
    const lines = editor.text.split("\n");

    if (cursor.line === 0 && cursor.column === 0) {
        return;
    }

    if (cursor.column > 0) {
        const line = lines[cursor.line];
        lines[cursor.line] = line.slice(0, cursor.column - 1) + line.slice(cursor.column);

        cursor.column--;
    } else {
        const previousLine = lines[cursor.line - 1];
        const currentLine = lines[cursor.line];

        cursor.line--;
        cursor.column = previousLine.length;
        lines[cursor.line] = previousLine + currentLine;
        lines.splice(cursor.line + 1, 1);
    }

    editor.text = lines.join("\n");
    saveEditor();
    render();
}

function handleEnter() {
    const cursor = editor.cursors[0];
    const lines = editor.text.split("\n");

    const currentLine = lines[cursor.line];

    const before = currentLine.slice(0, cursor.column);
    const after = currentLine.slice(cursor.column);

    lines[cursor.line] = before;
    lines.splice(cursor.line + 1, 0, after);

    editor.text = lines.join("\n");

    cursor.line++;
    cursor.column = 0;
    saveEditor();
    render();
}

function insertText(text) {
    const cursor = editor.cursors[0];
    const lines = editor.text.split("\n");

    const currentLine = lines[cursor.line];

    lines[cursor.line] = lines[cursor.line].slice(0, cursor.column) + text + lines[cursor.line].slice(cursor.column);

    editor.text = lines.join("\n");

    editor.cursors[0].column += text.length;

    saveEditor();
    render();
}

function handleArrowKey(key) {
    const lines = editor.text.split("\n");
    for (const cursor of editor.cursors) {
        if (key === "ArrowLeft") {
            if (cursor.column > 0) {
                cursor.column--;
            } else if (cursor.line > 0) {
                cursor.line--;
                cursor.column = lines[cursor.line].length;
            }
        }

        if (key === "ArrowRight") {
            if (cursor.column < lines[cursor.line].length) {
                cursor.column++;
            } else if (cursor.line < lines.length - 1) {
                cursor.line++;
                cursor.column = 0;
            }
        }

        if (key === "ArrowUp") {
            if (cursor.line > 0) {
                cursor.line--;
                cursor.column = Math.min(
                    cursor.column,
                    lines[cursor.line].length
                );
            }
        }

        if (key === "ArrowDown") {
            if (cursor.line < lines.length - 1) {
                cursor.line++;
                cursor.column = Math.min(
                    cursor.column,
                    lines[cursor.line].length
                );
            }
        }
    }


    render();
}

function keyDownListener(event) {


    if (event.key.startsWith("Arrow")) {
        event.preventDefault();
        handleArrowKey(event.key);
        return;
    }

    if (event.key === "Tab") {
        event.preventDefault();
        handleTab();
        return;
    }

    if (event.key === "Backspace") {
        event.preventDefault();
        handleBackspace();
        return;
    }

    if (event.key === "Enter") {
        event.preventDefault();
        handleEnter();
        return;
    }

    if (event.key.length === 1) {
        event.preventDefault();
        insertText(event.key);
        return;
    }
}


function main() {
    input.addEventListener("keydown", keyDownListener);

    input.addEventListener("focus", () => {
        editor.focused = true;
        render();
    });

    input.addEventListener("blur", () => {
        editor.focused = false;
        render();
    });


    input.addEventListener("mousedown", event => {
        const location = getLocationFromMouse(event);

        console.log(location);

        editor.cursors = [{
            position: 0, // todo
            ...location
        }];

        input.focus({ preventScroll: true });
        render();
    });

    render();
}

main()