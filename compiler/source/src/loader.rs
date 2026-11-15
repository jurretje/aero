use std::path::PathBuf;

use crate::{
    map::{SourceMap, SourceId},
    span::Span,
};

pub trait SourceResolver {
    type Request;

    fn root_request(&self, name: &str) -> Self::Request;

    fn child_request(&self, parent: SourceId, source: &str, span: Span) -> Self::Request;
}

pub trait FileProvider {
    type Request;
    type Error;

    fn load(&mut self, request: &Self::Request) -> Result<String, Self::Error>;
}

pub struct SourceLoader<P, R> {
    pub provider: P,
    pub resolver: R,
}

impl<P, R> SourceLoader<P, R>
where
    P: FileProvider,
    R: SourceResolver<Request = P::Request>,
{
    pub fn load_root(
        &mut self,
        sources: &mut SourceMap,
        name: &str,
    ) -> Result<SourceId, P::Error> {
        let request = self.resolver.root_request(name);
        let source = self.provider.load(&request)?;

        Ok(sources.insert_root(name.to_string(), source))
    }

    pub fn load_child(
        &mut self,
        sources: &mut SourceMap,
        parent: SourceId,
        span: Span,
    ) -> Result<SourceId, P::Error> {
        let request = self
            .resolver
            .child_request(parent, sources.get_source(parent), span);

        let source = self.provider.load(&request)?;
        Ok(sources.insert_source(parent, span, source))
    }
}

pub struct SystemFileProvider {
    pub root: PathBuf,
}

impl FileProvider for SystemFileProvider {
    type Request = PathBuf;
    type Error = std::io::Error;

    fn load(&mut self, request: &Self::Request) -> Result<String, Self::Error> {
        std::fs::read_to_string(self.root.join(request))
    }
}
