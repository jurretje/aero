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

type Color = {
    r: number;
    g: number;
    b: number;
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

type Theme = {
    keyword: Color;
    type_name: Color;
    identifier: Color;
    operator: Color;
    punctuation: Color;
};

export type CompilerOptions = {
    parse_costs: {
        insert: number;
        delete: number;
        replace: number;
    },
    theme: string
}

type InitResponse = {
    source: string;
    options: CompilerOptions;
    theme: Theme;
}

type OptionsResponse = {
    options: CompilerOptions;
    theme: Theme;
}

export let analysis_result: CompileResult | null = null;
export let compiler_options: CompilerOptions | null = null;

export function apply_theme(theme: Theme) {
    const root = document.documentElement;

    for (const [key, { r, g, b }] of Object.entries(theme)) {
        const name = key === "type_name" ? "type" : key;
        root.style.setProperty(`--syntax-${name}`, `rgb(${r}, ${g}, ${b})`);
    }
}

export async function init_request() {
    const response = await fetch("/api/init");

    if (!response.ok) {
        throw new Error(`Initialization failed: ${response.status}`);
    }

    const data: InitResponse = await response.json();

    console.log(data);

    compiler_options = data.options;
    apply_theme(data.theme);
    editor.text = data.source;

    await analyze();
}

export async function analyze() {
    try {
        analysis_result = await request_analysis(
            editor.text,
            editor.cursors
        );

        console.log(analysis_result);
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

export async function update_compiler_options(options: CompilerOptions) {
    const response = await fetch("/api/options", {
        method: "PUT",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ options })
    });

    if (!response.ok) {
        throw new Error(`Updating compiler options failed: ${response.status}`);
    }

    const data: OptionsResponse = await response.json();

    compiler_options = data.options;
    apply_theme(data.theme);
}