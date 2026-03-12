use crate::db_executor::DBExecutorTrait;

pub struct DBExecutor {}

impl DBExecutorTrait for DBExecutor {
    type FileInfo = ();
    type Err = ();

    async fn save_file_info(&self, file_info: &Self::FileInfo) -> Result<(), Self::Err> {
        Ok(())
    }
}

