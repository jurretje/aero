import { Cursor, editor } from "../editor/state.js";

type CompileResult = {
    success: boolean;
    tokens: StyledToken[] 
};

export type StyledToken = {
    kind: string;
    span: any;
    style: any;
};

type Span = {
    start: any;
    end: any;
};

export let compile_result: CompileResult | null = null;

export async function analyze() {
    try {
        compile_result = await request_analysis(
            editor.text,
            editor.cursors
        );

        console.log(compile_result);
    } catch (error) {
        console.error(error);
    }
}

export async function request_analysis(source: string, cursors: Cursor[]) {
    const response = await fetch("/api/analysis", {
        method: "POST",

        headers: {
            "Content-Type": "application/json",
        },

        body: JSON.stringify({
            source,
            cursors,
        }),
    });

    if (!response.ok) {
        throw new Error(
            `Compiler request failed: ${response.status}`
        );
    }

    return await response.json();
}