#[derive(Debug)]
pub enum CompilationError {
    UnidentifiedVariable { identifier: String },
}
