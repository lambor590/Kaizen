use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use thiserror::Error;

pub mod cleaner;

pub type KaizenResult<T> = std::result::Result<T, KaizenSerializableError>;

#[derive(Error, Debug)]
pub enum KaizenSerializableError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
}

impl Serialize for KaizenSerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state: <S as Serializer>::SerializeStruct =
            serializer.serialize_struct("KaizenSerializableError", 1)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tools")
        .invoke_handler(tauri::generate_handler![cleaner::run_cleaner])
        .build()
}
