use crate::auth::Credentials;

use hyper::http::{Extensions, HeaderValue};
use hyper::{HeaderMap, Uri};
use rust_utils::default::default;
use std::cell::RefCell;
#[derive(Debug)]
#[non_exhaustive]
pub struct S3Request<T> {
    /// Operation input
    pub input: T,

    /// Identity information.
    ///
    /// `None` means anonymous request.
    pub credentials: Option<Credentials>,

    /// Request extensions
    ///
    /// It is used to pass custom data between middlewares.
    pub extensions: Extensions,

    // Headers
    pub headers: HeaderMap<HeaderValue>,

    // Raw URI
    pub uri: Uri,

    // passwordd
    pub password: RefCell<String>,
}

impl<T> S3Request<T> {
    pub fn new(input: T ) -> Self {
        Self {
            input,
            credentials: default(),
            extensions: default(),
            headers: default(),
            uri: default(),
            password: default(),
        }
    }
    pub fn set_string(self, new_password: String) {
        *self.password.borrow_mut() = new_password.clone();
    }

    pub fn map_input<U>(self, f: impl FnOnce(T) -> U) -> S3Request<U> {
        S3Request {
            input: f(self.input),
            credentials: self.credentials,
            extensions: self.extensions,
            headers: self.headers,
            uri: self.uri,
            password: self.password,
        }
    }
}
