use pipeline::Compiler;

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let compiler = Compiler::new();

    let input = std::fs::read_to_string("examples/main.ae")?;

    let output = compiler.compile(&input);

    if let Some(node) = output.parsed {
        let pretty_printed_node = node.highlighted(&input);
        println!("{}", pretty_printed_node);
    }

    Ok(())
}
