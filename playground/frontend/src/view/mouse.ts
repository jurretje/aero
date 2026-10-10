import { editor } from "../editor/state.js";

export function get_location_from_mouse(event: MouseEvent, viewport: HTMLElement) {
    const rect = viewport.getBoundingClientRect();

    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const lines = editor.text.split("\n");

    const lineHeight =
        parseFloat(getComputedStyle(viewport).lineHeight);

    let line = Math.floor(y / lineHeight);

    line = Math.max(
        0,
        Math.min(line, lines.length - 1)
    );

    const text = lines[line] ?? "";

    const style = getComputedStyle(viewport);

    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d")!;

    context.font = `${style.fontSize} ${style.fontFamily}`;

    const charWidth =
        context.measureText("M").width;

    let column = Math.round(x / charWidth);

    column = Math.max(
        0,
        Math.min(column, text.length)
    );

    return {
        line,
        column,
    };
}