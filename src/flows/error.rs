#[derive(Debug)]
pub enum FlowError {
    CaNotFound,
    CertNotFound,
    ValidationError(String),
    SomethingWentWrong(String),
}
