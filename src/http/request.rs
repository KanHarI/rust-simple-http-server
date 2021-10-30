use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::Method;
use super::method::MethodError;
use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf> {
    method: super::Method,
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        // ? Will return from mid function
        let decoded_request = str::from_utf8(buffer)?;
        let (method, decoded_request) =
            get_next_word(decoded_request).ok_or(ParseError::InvalidRequest)?;
        let (path_and_query_string, decoded_request) =
            get_next_word(decoded_request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(decoded_request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        let mut path = path_and_query_string;

        // Ignore the other options - combined match and if
        if let Some(i) = path_and_query_string.find('?') {
            path = &path_and_query_string[..i];
            query_string = Some(QueryString::from(&path_and_query_string[(i+1)..]));
        }
        
        Ok(Self {
            path: path,
            query_string: query_string,
            method: method
        })
    }
}

fn get_next_word(decoded_request: &str) -> Option<(&str, &str)> {
    for (i, c) in decoded_request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&decoded_request[..i], &decoded_request[(i + 1)..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
