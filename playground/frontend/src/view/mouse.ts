import { editor } from "../editor/state.js";

export function get_location_from_mouse(event: MouseEvent, viewport: HTMLElement) {
    const lines = [...viewport.children] as HTMLElement[];

    if (lines.length === 0) {
        return { line: 0, column: 0 };
    }

    const y = event.clientY;

    let line = 0;
    let min_distance = Infinity;

    for (let i = 0; i < lines.length; i++) {
        const rect = lines[i]!.getBoundingClientRect();
        const center = rect.top + rect.height / 2;
        const distance = Math.abs(y - center);
        if (distance < min_distance) {
            min_distance = distance;
            line = i;
        }
    }

    const text = editor.text.split("\n")[line] ?? "";
    const line_element = lines[line]!;

    const style = getComputedStyle(line_element);
    
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d")!;

    context.font = `${style.fontSize} ${style.fontFamily}`;

    const char_width = context.measureText("M").width;
    let x = event.clientX - viewport.getBoundingClientRect().left;
    let column = Math.round(x / char_width);
    column = Math.max(0, Math.min(column, text.length));

    return {
        line,
        column,
    };
}