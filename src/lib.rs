mod config;
mod export;
mod record;

pub use config::{Config, ZoneConfig};
pub use export::get;
pub use record::Record;

use reqwest::Method;

pub async fn apply(state: State, config: Config) -> Result<(), Error> {
    let zone_id = &config.zone.id;
    let zone_name = &config.zone.name;
    let records_resp: CfResult<Vec<Record>> =
        state.get(&format!("/zones/{zone_id}/dns_records")).await?;
    let existing: Vec<Record> = records_resp
        .result
        .into_iter()
        .map(|v| v.normalize_name(zone_name))
        .collect();
    let mut wanted = config.records.clone();
    if config.zone.restrictive_email {
        add_restrictive_email(&mut wanted);
    }
    Ok(())
}

fn add_restrictive_email(records: &mut Vec<Record>) {
    let restrictive_spf = Record::restrictive_spf();
    let restrictive_dkim = Record::restrictive_dkim();
    let restrictive_dmarc = Record::restrictive_dmarc();
    if !records.contains(&restrictive_spf) {
        records.push(restrictive_spf);
    }
    if !records.contains(&restrictive_dkim) {
        records.push(restrictive_dkim);
    }
    if !records.contains(&restrictive_dmarc) {
        records.push(restrictive_dmarc);
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CfResult<T> {
    pub errors: Vec<CfMessage>,
    pub messages: Vec<CfMessage>,
    pub success: bool,
    pub result: T,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CfMessage {
    code: u32,
    message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serde-json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("serde-toml serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Invalid input: {0} for struct {1}")]
    Validate(&'static str, &'static str),
    #[error("{0}")]
    Generic(String),
    #[error("Cloudflare API returned no data where some was expected")]
    NoData,
}

#[derive(Clone, Debug)]
pub struct State {
    pub api_token: String,
    pub http: reqwest::Client,
}

impl State {
    pub async fn get<R: for<'de> serde::Deserialize<'de>>(&self, route: &str) -> Result<R, Error> {
        self.req::<(), R>(Method::GET, route, None).await
    }
    pub async fn post<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, Error> {
        self.req(Method::POST, route, Some(&body)).await
    }
    pub async fn patch<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, Error> {
        self.req(Method::PATCH, route, Some(&body)).await
    }
    pub async fn put<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, Error> {
        self.req(Method::PUT, route, Some(&body)).await
    }
    pub async fn req<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        meth: Method,
        route: &str,
        maybe_body: Option<&I>,
    ) -> Result<R, Error> {
        let mut req = self.http.request(meth, format!("{API_ROOT}{route}"));
        let req_body: Option<String>;
        if let Some(body_model) = maybe_body {
            let req_body_ser = serde_json::to_string(body_model)?;
            req_body = Some(req_body_ser.clone());
            req = req
                .body(req_body_ser)
                .header("Content-Type", "application/json; charset=utf-8");
        } else {
            req_body = None;
        }
        let req_body_fmt = req_body.unwrap_or_default();
        let resp = req.send().await?;
        let potential_err = resp.error_for_status_ref().err();
        let resp_body = resp.text().await?;
        if let Some(err) = potential_err {
            return Err(Error::Generic(format!(
                "Got error {err} (response body `{resp_body}`, request body `{req_body_fmt}`) while making request to {route}",
            )));
        }
        let json: R = match serde_json::from_str(&resp_body) {
            Ok(v) => v,
            Err(err) => {
                return Err(Error::Generic(format!(
                    "Got error {err} while trying to deserialize `{resp_body}` as json"
                )))
            }
        };
        Ok(json)
    }
}

const API_ROOT: &str = "https://api.cloudflare.com/client/v4";
