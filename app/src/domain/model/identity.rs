use crate::derive_newtype_serde;
use crate::error::*;
use serde::*;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug)]
pub enum IdentityError {
    ParseError,
}

impl IServiceError for IdentityError {
    fn error_type(&self) -> String {
        use IdentityError::*;

        match self {
            ParseError => "parse_error",
        }
        .to_string()
    }

    fn status_code(&self) -> http::StatusCode {
        use IdentityError::*;

        match self {
            ParseError => http::StatusCode::BAD_REQUEST,
        }
    }
}
macro_rules! impl_id {
    ($ty:ty) => {
        derive_newtype_serde!($ty, String; <$ty>::from_raw, "impl_id!: invalid string representation");

        impl $ty {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4().to_string())
            }

            #[allow(dead_code)]
            pub fn to_raw(&self) -> String {
                self.0.clone()
            }

            pub fn from_raw(s: String) -> Result<Self> {
                Uuid::parse_str(&s).map_err(|_| ServiceError::only(IdentityError::ParseError))?;
                Ok(Self(s))
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct TweetID(pub String);
impl_id!(TweetID);
