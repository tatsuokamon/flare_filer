#[derive(thiserror::Error)]
pub enum EngineErr<SaverErr, DBErr> {
    #[error("EngineErr: FromSaver {0}")]
    FromSaver(SaverErr),

    #[error("EngineErr: FromDB {0}")]
    FromDB(DBErr),
}
