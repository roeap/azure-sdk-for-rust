use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptEncoding<'a>(&'a str);

impl<'a> AcceptEncoding<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a, S> From<S> for AcceptEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for AcceptEncoding<'a> {
    fn name(&self) -> headers::HeaderName {
        http::header::ACCEPT_ENCODING.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
