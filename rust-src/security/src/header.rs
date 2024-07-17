
use actix_web::error::ParseError;
use actix_web::http::header;
use actix_web::http::header::{Header, HeaderName, HeaderValue, InvalidHeaderName, TryIntoHeaderValue};
use actix_web::HttpMessage;
use log::error;

pub struct Authorization {
    pub token_value: String,
    header_value: HeaderValue,
}

impl Authorization {
    fn new(token_value: String, header_value: HeaderValue) -> Authorization{
        Authorization{
            token_value,
            header_value,
        }
    }
}

impl TryIntoHeaderValue for Authorization {
    type Error = InvalidHeaderName;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        Ok(self.header_value)
    }
}

impl Header for Authorization{
    fn name() -> HeaderName {
        header::AUTHORIZATION
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let header_value = msg.headers().get(Self::name());
        match header_value {
            None => {
                error!("no set header Authorization");
                Err(ParseError::Header)
            }
            Some(header_value) => {
                let token_str = header_value.to_str().unwrap();
               Ok( Authorization::new(token_str.to_string(), header_value.clone()))
            }
        }
    }
}



