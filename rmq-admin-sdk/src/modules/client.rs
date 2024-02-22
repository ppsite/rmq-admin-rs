use base64::prelude::*;
use reqwest::{header::HeaderMap, Response};
use std::{collections::HashMap, error::Error, time};

#[derive(Clone)]
pub struct Rabbitmq {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub timeout: u8,
}

impl Rabbitmq {
    pub fn new(host: String, port: u16, username: String, password: String, timeout: u8) -> Self {
        Self {
            host,
            port,
            username,
            password,
            timeout,
        }
    }

    pub async fn get(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
        params: &[(&str, &str)],
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }

        let auth_token =
            BASE64_STANDARD.encode(format!("{}:{}", self.username, self.password).as_bytes());
        _headers.insert("Authorization", format!("Basic {}", auth_token).parse()?);

        let url: String = format!("{}:{}/{}", self.host, self.port, uri).parse()?;

        let client = client
            .get(url)
            .query(params)
            .headers(_headers)
            .timeout(time::Duration::from_secs(self.timeout.into()))
            .send()
            .await?;
        Ok(client)
    }

    pub async fn post(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
        params: &[(&str, &str)],
        body: Option<HashMap<String, String>>,
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut _headers = HeaderMap::new();
        match headers {
            Some(h) => _headers.extend(h),
            None => {}
        }

        let auth_token =
            BASE64_STANDARD.encode(format!("{}:{}", self.username, self.password).as_bytes());
        _headers.insert("Authorization", format!("Basic {}", auth_token).parse()?);

        let url: String = format!("{}:{}/{}", self.host, self.port, uri).parse()?;

        let mut _body = HashMap::new();
        match body {
            Some(b) => _body.extend(b),
            None => {}
        }
        let response = client
            .post(url)
            .query(params)
            .headers(_headers)
            .timeout(time::Duration::from_secs(self.timeout.into()))
            .json(&_body)
            .send()
            .await?;
        Ok(response)
    }
}
