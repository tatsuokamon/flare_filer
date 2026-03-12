use std::sync::Arc;

use crate::{db_executor::DBExecutorTrait, file_saver::FileSaver};

pub type EngineState<Saver, DBExecutor> = Arc<EngineStateSrc<Saver, DBExecutor>>;

pub struct EngineStateSrc<Saver, DBExecutor>
where
    Saver: FileSaver,
    DBExecutor: DBExecutorTrait,
{
    pub saver: Saver,
    pub db: DBExecutor,
}
