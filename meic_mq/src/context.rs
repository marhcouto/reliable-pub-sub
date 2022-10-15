use std::io::BufReader;

use serde::{ Serialize, de::DeserializeOwned };

const PUB_STORAGE_PATH: &str = "./data/pub/";
const SUB_STORAGE_PATH: &str = "./data/sub/";

pub enum ContextIOError {
    ErrorCreatingDirectory(String),
    ErrorWritingToFile(String),
    ErrorCantFindFile(String),
    ErrorReadingFile(String)
}

trait FileWritable {
    fn build_path(&self) -> String;
}

fn save<T>(context: &T) -> Result<(), ContextIOError>
where T: Serialize + FileWritable {
    if let Err(err) = std::fs::create_dir_all(PUB_STORAGE_PATH) {
        return Err(ContextIOError::ErrorCreatingDirectory(err.to_string()));
    }
    let data_bytes = match bson::to_vec(context) {
        Ok(val) => val,
        Err(err) => return Err(ContextIOError::ErrorWritingToFile(err.to_string()))
    };
    match std::fs::write(context.build_path(), data_bytes) {
        Ok(_) => Ok(()),
        Err(err) => Err(ContextIOError::ErrorWritingToFile(err.to_string()))
    }
}

fn read<T>(path: String) -> Result<T, ContextIOError> 
where T: DeserializeOwned {
    let file = match std::fs::File::open(path) {
        Ok(val) => val,
        Err(err) => return Err(ContextIOError::ErrorCantFindFile(err.to_string()))
    };
    let file_reader = BufReader::new(file);
    match bson::from_reader(file_reader) {
        Ok(val) => Ok(val),
        Err(err) => Err(ContextIOError::ErrorReadingFile(err.to_string()))
    }
}

pub mod subscriber;
pub mod publisher;
