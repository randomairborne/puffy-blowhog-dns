mod config;
mod record;

pub use config::{Config, ZoneConfig};
pub use record::Record;

pub async fn entrypoint(state: State) -> Result<(), Error> {
    let zone_id = &state.config.zone.id;
    let records_resp: CfListBody<Record> =
        state.get(&format!("/zones/{zone_id}/dns_records")).await?;
    let records = records_resp.result;
    if state.config.zone.restrictive_email {
        let restrictive_spf = Record::restrictive_spf();
        let restrictive_dkim = Record::restrictive_dkim();
        let restrictive_dmarc = Record::restrictive_dmarc();
        if !records.contains(&restrictive_spf) {}
        let _: CfNewRecordBody = state
            .post(&format!("/zones/{zone_id}/dns_records"), restrictive_spf)
            .await?;
        let _: CfNewRecordBody = state
            .post(&format!("/zones/{zone_id}/dns_records"), restrictive_dkim)
            .await?;
        let _: CfNewRecordBody = state
            .post(&format!("/zones/{zone_id}/dns_records"), restrictive_dmarc)
            .await?;
    }
    println!("{records:#?}");
    Ok(())
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CfListBody<T> {
    result: Vec<T>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CfNewRecordBody {
    result: Record,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Invalid input: {0} for struct {1}")]
    Validate(&'static str, &'static str)
}

#[derive(Clone, Debug)]
pub struct State {
    pub config: Config,
    pub api_token: String,
    pub http: reqwest::Client,
}

impl State {
    pub async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
    ) -> Result<T, reqwest::Error> {
        self.http
            .get(format!("{API_ROOT}{route}"))
            .send()
            .await?
            .json::<T>()
            .await
    }
    pub async fn post<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, reqwest::Error> {
        self.http
            .post(format!("{API_ROOT}{route}"))
            .json(&body)
            .send()
            .await?
            .json::<R>()
            .await
    }
    pub async fn patch<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, reqwest::Error> {
        self.http
            .patch(format!("{API_ROOT}{route}"))
            .json(&body)
            .send()
            .await?
            .json::<R>()
            .await
    }
    pub async fn put<I: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        route: &str,
        body: I,
    ) -> Result<R, reqwest::Error> {
        self.http
            .patch(format!("{API_ROOT}{route}"))
            .json(&body)
            .send()
            .await?
            .json::<R>()
            .await
    }
}

const API_ROOT: &str = "https://api.cloudflare.com/client/v4";
