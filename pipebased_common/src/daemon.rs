use crate::{PipeManager, RepositoryManager};

pub struct Daemon<'a> {
    repository_manager: RepositoryManager<'a>,
    pipe_manager: PipeManager<'a>,
}

impl<'a> Daemon<'a> {}
