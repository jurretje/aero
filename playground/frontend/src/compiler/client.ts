import { Cursor, editor } from "../editor/state.js";

export type CompileResult = {
    success: boolean;
    tokens: StyledToken[];
    ast: Node | null;
    grammar_graph: GrammarGraph | null;
};

type Node = {
    kind: any;
    span: Span;
    children: Node[];
};

type GrammarGraph = {
    states: State[];
    start: State
}

type State = {
    transitions: Transition[]
}

type Transition = {

}

type TokenTransition = {
    kind: string;
    target: number;
}

export type StyledToken = {
    kind: string;
    span: Span;
    style: string;
};

export type Span = {
    start: Location;
    end: Location;
};

export type Location = {
    pos: number;
    line: number;
    column: number;
}

export let compile_result: CompileResult | null = null;

export async function analyze() {
    try {
        compile_result = await request_analysis(
            editor.text,
            editor.cursors
        );

        console.log(compile_result!.grammar_graph);
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