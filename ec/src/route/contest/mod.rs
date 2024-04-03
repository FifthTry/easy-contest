pub mod submission;
pub use submission::{DeleteSubmissionPayload, SubmissionPayload, Submissions};

pub struct Contest {
    in_: ft_sdk::In,
    ud: ft_sdk::UserData,
    conn: ft_sdk::PgConnection,
}

impl ft_sdk::Layout for Contest {
    type Error = ContestError;

    fn from_in(in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        let conn = ft_sdk::default_pg()?;

        if in_.ud.is_none() {
            return Err(Self::Error::Unauthorized(
                "You must be logged in to view this page".to_string(),
            ));
        }

        let ud = in_.ud.clone().expect("user must exist in now");

        Ok(Self { in_, conn, ud })
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::json!({
            "page": page,
        }))
    }

    fn render_error(err: Self::Error) -> http::Response<bytes::Bytes> {
        match err {
            ContestError::FormError(errors) => {
                ft_sdk::println!("form error: {errors:?}");
                ft_sdk::json_response(serde_json::json!({"errors": errors}))
            }
            ContestError::Sdk(error) => {
                ft_sdk::server_error!("sdk error: {error:?}")
            }
            ContestError::Diesel(error) => {
                ft_sdk::server_error!("diesel error: {error:?}")
            }
            ContestError::CantDeserializeInput(message) => {
                ft_sdk::server_error!("serde error: {message:?}")
            }
            ContestError::Unauthorized(message) => {
                ft_sdk::not_found!("unauthorized error: {message}")
            }
            ContestError::UsageError(message) => {
                ft_sdk::not_found!("{message}")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContestError {
    #[error("form error: {0:?}")]
    FormError(std::collections::HashMap<String, String>),
    #[error("sdk error: {0}")]
    Sdk(#[from] ft_sdk::Error),
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("cant deserialize input: {0}")]
    CantDeserializeInput(#[from] serde_json::Error),
    #[error("not authorised: {0}")]
    Unauthorized(String),
    #[error("usage error: {0}")]
    UsageError(String),
}

impl ContestError {
    pub fn form_error(field: &str, error: &str) -> Self {
        Self::FormError(std::collections::HashMap::from([(
            field.to_string(),
            error.to_string(),
        )]))
    }
}
