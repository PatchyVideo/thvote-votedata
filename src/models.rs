
use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ErrorResponse {
	code: u16,
	error: String,
	message: String,
	detail: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ServiceError {
	Unknown{ detail: String },
}
impl std::error::Error for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ServiceError {
	pub fn name(&self) -> String {
		match self {
			ServiceError::Unknown{..} => "Unknown".to_string(),
		}
	}
}
impl ResponseError for ServiceError {
	fn status_code(&self) -> StatusCode {
		match self {
			ServiceError::Unknown{..} => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}

	fn error_response(&self) -> HttpResponse {
		let status_code = self.status_code();
        let detail = match self {
			ServiceError::Unknown { detail } => Some(detail.clone()),
			_ => None
		};
		let error_response = ErrorResponse {
			code: status_code.as_u16(),
			message: self.to_string(),
			error: self.name(),
			detail: detail
		};
		HttpResponse::build(status_code).json(error_response)
	}
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub altnames: Vec<String>,
    pub title: String,
    pub image: String,
    pub color: String,
    pub first_appeared_in: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllCharacters {
    pub data: Vec<Character>
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Music {
    pub name: String,
    pub first_appeared_in: String,
    pub corresponding_character: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllMusics {
    pub data: Vec<Music>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Work {
    pub name: String,
    pub altnames: Vec<String>,
    pub date: String,
    pub image: String,
    pub kind: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllWorks {
    pub data: Vec<Work>
}
