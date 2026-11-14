use std::collections::HashMap;

use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextRef {
    pub source: SourceId,
    pub span: Span,
}

pub enum FileName {
    Root(usize),
    Source { source: SourceId, span: Span },
}

pub struct SourceFile {
    name: FileName,
    source: String,
}

#[derive(Default)]
pub struct SourceMap {
    root_names: Vec<String>,
    files: HashMap<SourceId, SourceFile>,
}

impl SourceMap {
    pub fn insert_root(&mut self, path: String, source: String) -> SourceId {
        let index = self.root_names.len();
        self.root_names.push(path);

        let id = SourceId(self.files.len());
        self.files.insert(
            id,
            SourceFile {
                name: FileName::Root(index),
                source,
            },
        );

        id
    }

    pub fn insert_source(&mut self, parent: SourceId, span: Span, source: String) -> SourceId {
        let id = SourceId(self.files.len());

        self.files.insert(
            id,
            SourceFile {
                name: FileName::Source {
                    source: parent,
                    span,
                },
                source,
            },
        );

        id
    }

    pub fn source(&self, id: SourceId) -> &str {
        &self.files[&id].source
    }
}
