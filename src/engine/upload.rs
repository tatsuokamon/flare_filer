use crate::{
    db_executor::DBExecutorTrait,
    engine::{err::EngineErr, state::EngineState},
    file_saver::FileSaver,
};
use axum::{
    body::Bytes,
    extract::{Multipart, State},
    response::IntoResponse,
};

pub fn make_upload_handler<Saver, DB>()
-> impl Fn(Multipart, State<EngineState<Saver, DB>>) -> impl Future<Output = impl IntoResponse>
where
    Saver: FileSaver,
    DB: DBExecutorTrait,
{
    |multi_part: Multipart, State(state): State<EngineState<Saver, DB>| {
    }
}

async fn upload_inner<Saver, DBExecutor>(
    file: &str,
    data: Bytes,
    saver: &Saver,
    db: &DBExecutor,
) -> Result<(), EngineErr<Saver::Err, DBExecutor::Err>>
where
    Saver: FileSaver,
    DBExecutor: DBExecutorTrait,
{
    let file_info = db
        .ready_file_info(&file, &data)
        .await
        .map_err(EngineErr::FromDB)?;

    saver
        .send_file(&file, data.into())
        .await
        .map_err(EngineErr::FromSaver)?;

    db.save_file_info(&file_info)
        .await
        .map_err(EngineErr::FromDB)?;

    Ok(())
}
