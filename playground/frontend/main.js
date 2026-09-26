const viewport = document.getElementById("viewport");
const input = document.getElementById("input");


const STORAGE_KEY = "aero-playground-source";
const SETTINGS = {
    tabSize: 4
};

const editor = {
    text: localStorage.getItem(STORAGE_KEY) ?? "",
    cursors: [{ position: 0, anchor: 0 }],
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

function positionToLocation(position) {
    const before = editor.text.slice(0, position);

    const line = before.split("\n").length - 1;
    const lastNewline = before.lastIndexOf("\n");

    const column =
        lastNewline === -1
            ? position
            : position - lastNewline - 1;

    return { line, column };
}

function locationToPosition(line, column) {
    const lines = editor.text.split("\n");

    let position = 0;
    for (let i = 0; i < line; i++) {
        position += lines[i].length + 1;
    }

    return position + column;
}

function selectionStart(cursor) {
    return Math.min(cursor.position, cursor.anchor);
}

function selectionEnd(cursor) {
    return Math.max(cursor.position, cursor.anchor);
}

function hasSelection(cursor) {
    return cursor.position !== cursor.anchor;
}

function selectedText(cursor) {
    return editor.text.slice(
        selectionStart(cursor),
        selectionEnd(cursor)
    );
}

function selectAll() {
    console.log("select all");

    editor.cursors = [{
        position: editor.text.length,
        anchor: 0
    }];

    render();
}

function saveEditor() {
    localStorage.setItem(STORAGE_KEY, editor.text);
}

function renderLine(text, lineNumber) {
    const line = document.createElement("div");
    line.className = "line-of-code";

    for (let i = 0; i < text.length; i++) {
        const char = document.createElement("span");
        char.textContent = text[i];

        const position = locationToPosition(lineNumber, i);

        for (const cursor of editor.cursors) {
            const start = selectionStart(cursor);
            const end = selectionEnd(cursor);

            if (position >= start && position < end) {
                char.classList.add("selected");
            }
        }

        line.appendChild(char);
    }

    for (const cursor of editor.cursors) {
        const location = positionToLocation(cursor.position);

        if (location.line === lineNumber) {
            const caret = document.createElement("span");
            caret.className = "cursor";
            caret.style.left = `${location.column}ch`;

            line.appendChild(caret);
        }
    }

    return line;
}


function render() {
    viewport.innerHTML = "";

    const lines = editor.text.split("\n");

    for (let i = 0; i < lines.length; i++) {
        viewport.appendChild(
            renderLine(lines[i], i)
        );
    }
}

function handleEnter() {
    insertText("\n");
}

function handleTab() {
    for (const cursor of editor.cursors) {
        const { column } = positionToLocation(cursor.position);
        const count = SETTINGS.tabSize - (column % SETTINGS.tabSize);
        insertText(" ".repeat(count));
    }
}

function handleBackspace() {
    const edits = editor.cursors
        .map(cursor => {
            const start = selectionStart(cursor);
            const end = selectionEnd(cursor);

            if (start !== end) {
                cursor.position = start;
                cursor.anchor = start;

                return {
                    start,
                    end,
                    text: ""
                };
            }

            if (cursor.position === 0) {
                return null;
            }

            const position = cursor.position;

            cursor.position--;
            cursor.anchor--;

            return {
                start: position - 1,
                end: position,
                text: ""
            };
        })
        .filter(Boolean);

    replaceRanges(edits);
    normalizeCursors();

    saveEditor();
    render();
}

function insertText(text) {
    const cursors = [...editor.cursors]
        .sort((a, b) => selectionStart(b) - selectionStart(a));

    for (const cursor of cursors) {
        const start = selectionStart(cursor);
        const end = selectionEnd(cursor);

        editor.text =
            editor.text.slice(0, start) +
            text +
            editor.text.slice(end);

        const position = start + text.length;

        cursor.position = position;
        cursor.anchor = position;
    }

    saveEditor();
    render();
}

function replaceRanges(edits) {
    edits.sort((a, b) => b.start - a.start);

    for (const edit of edits) {
        editor.text =
            editor.text.slice(0, edit.start) +
            edit.text +
            editor.text.slice(edit.end);
    }
}

function nextWordBoundary(position) {
    const length = editor.text.length;

    let i = position;

    while (i < length && /\s/.test(editor.text[i])) {
        i++;
    }

    while (i < length && /[\w]/.test(editor.text[i])) {
        i++;
    }

    return i;
}

function handleCtrlDelete() {
    const cursor = editor.cursors[0];

    if (hasSelection(cursor)) {
        insertText("");
        return;
    }

    const end = nextWordBoundary(cursor.position);

    editor.text =
        editor.text.slice(0, cursor.position) +
        editor.text.slice(end);

    saveEditor();
    render();
}

function handleArrowKey(key, selecting) {
    for (const cursor of editor.cursors) {
        let position = cursor.position;

        if (key === "ArrowLeft") {
            position = Math.max(0, position - 1);
        }

        if (key === "ArrowRight") {
            position = Math.min(editor.text.length, position + 1);
        }

        if (key === "ArrowUp") {
            const { line, column } = positionToLocation(position);

            if (line > 0) {
                const newLine = line - 1;
                const newColumn = Math.min(
                    column,
                    editor.text.split("\n")[newLine].length
                );

                position = locationToPosition(newLine, newColumn);
            }
        }

        if (key === "ArrowDown") {
            const { line, column } = positionToLocation(position);
            const lines = editor.text.split("\n");

            if (line < lines.length - 1) {
                const newLine = line + 1;
                const newColumn = Math.min(
                    column,
                    lines[newLine].length
                );

                position = locationToPosition(newLine, newColumn);
            }
        }

        cursor.position = position;

        if (!selecting) {
            cursor.anchor = position;
        }
    }

    normalizeCursors();
    render();
}

function addCursor(position) {
    if (editor.cursors.some(cursor => cursor.position == position)) {
        return;
    }

    editor.cursors.push({
        position,
        anchor: position
    })
}

function keyDownListener(event) {
    const key = event.key.toLowerCase();

    if (event.ctrlKey && !event.shiftKey && key === "a") {
        event.preventDefault();
        selectAll();
        return;
    }

    if (event.ctrlKey && !event.shiftKey && key === "delete") {
        event.preventDefault();
        handleCtrlDelete();
        return;
    }

    if (event.key.startsWith("Arrow")) {
        event.preventDefault();
        handleArrowKey(event.key, event.shiftKey);
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

    if (
        event.key.length === 1 &&
        !event.ctrlKey &&
        !event.metaKey &&
        !event.altKey
    ) {
        event.preventDefault();
        insertText(event.key);
        return;
    }
}

function normalizeCursors() {
    const seen = new Set();

    editor.cursors = editor.cursors.filter(cursor => {
        if (seen.has(cursor.position)) {
            return false;
        }

        seen.add(cursor.position);
        return true;
    });
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

        const position = locationToPosition(
            location.line,
            location.column
        );

        if (event.altKey) {
            addCursor(position);
        } else {
            editor.cursors = [{
                position,
                anchor: position
            }];
        }

        input.focus({ preventScroll: true });
        render();
    });

    render();
}

main()