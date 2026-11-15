pub struct GrammarExtension {
    pub id: ExtensionId,
    pub dependencies: Vec<ExtensionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtensionId(pub usize);

pub struct ExtensionRegistry {}

impl ExtensionRegistry {
    pub fn register(&mut self, extension: GrammarExtension) {
        todo!()
    }

    pub fn get(&self, id: &ExtensionId) -> Option<&GrammarExtension> {
        todo!()
    }

    pub fn resolve_dependencies(&self) {
        todo!()
    }

    pub fn compile(&self) -> Result<GrammarRegistry, ()> {
        todo!()
    }
}

pub struct GrammarRegistry;
