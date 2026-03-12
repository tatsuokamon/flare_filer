mod err;
mod state;
mod upload;

use std::sync::Arc;

use axum::{Router, routing::post};
pub use err::EngineErr;
pub use state::EngineState;

use crate::{
    db_executor::DBExecutorTrait,
    engine::{state::EngineStateSrc, upload::upload_handler},
    file_saver::FileSaver,
    shared_architect::BuilderTrait,
};

struct RouterBuilder<Saver, DB>
where
    Saver: FileSaver,
    DB: DBExecutorTrait,
{
    pub saver: Saver,
    pub db: DB,
}

#[derive(thiserror::Error)]
pub enum RouterBuildErr<DBErr, SaverErr> {
    #[error("RouterBuildErr: FromDB {0}")]
    FromDB(DBErr),

    #[error("RouterBuildErr: FromSaver{0}")]
    FromSaver(SaverErr),
}

impl<Saver, DB> BuilderTrait for RouterBuilder<Saver, DB>
where
    Saver: FileSaver,
    DB: DBExecutorTrait,
{
    type Err = RouterBuildErr<Saver::Err, DB::Err>;
    type Target = Router;

    fn build(self) -> impl Future<Output = Result<Self::Target, Self::Err>> {
        async move {
            let state = Arc::new(EngineStateSrc {
                saver: self.saver,
                db: self.db,
            });

            Ok(Router::new()
                .route("/upload", post(upload_handler::<Saver, DB>))
                .with_state(state))
        }
    }
}
