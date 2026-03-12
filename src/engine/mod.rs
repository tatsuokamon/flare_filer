mod err;
mod state;
mod upload;

use std::sync::Arc;

use axum::{
    Router,
    extract::{Multipart, State, multipart},
    response::IntoResponse,
    routing::post,
};
pub use err::EngineErr;
pub use state::EngineState;

use crate::{
    cloud_flare_filer::CloudFlareFiler,
    db_executor::{DBExecutor, DBExecutorTrait},
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
    type Err = ();
    type Target = Router<Arc<EngineStateSrc<Saver, DBExecutor>>>;

    fn build(self) -> impl Future<Output = Result<Self::Target, Self::Err>> {
        async {
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

// impl BuilderTrait for RouterBuilder<CloudFlareFiler, DBExecutor> {
//     type Err = ();
//     type Target = Router<Arc<EngineState<CloudFlareFiler, DBExecutor>>>;
//
//     fn build(self) -> impl Future<Output = Result<Self::Target, Self::Err>> {
//         async {
//             let state = Arc::new(EngineStateSrc { saver: self.saver, db: self.db });
//             Ok(Router::new()
//                 .route(
//                     "/upload",
//                     post(upload_handler::<CloudFlareFiler, DBExecutor>),
//                 )
//                 .with_state(state))
//         }
//     }
// }
