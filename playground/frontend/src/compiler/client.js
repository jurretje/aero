export async function request_analysis(source, cursors) {
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