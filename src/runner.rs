use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct RunnerContext {
    pub(crate) programmatic: Option<bool>,
    pub(crate) has_lock: Option<bool>,
    pub(crate) cwd: Option<PathBuf>,
}
