use my_http_server::HttpFailResult;

use crate::flows::FlowError;

impl From<FlowError> for HttpFailResult {
    fn from(value: FlowError) -> Self {
        match value {
            FlowError::CaNotFound => Self::as_forbidden("Ca not found".to_string().into()),
            FlowError::CertNotFound => {
                Self::as_forbidden("Certification is not found".to_string().into())
            }
            FlowError::SomethingWentWrong(err) => Self::as_forbidden(err.to_string().into()),
        }
    }
}
