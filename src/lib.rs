mod config;
mod record;

pub use config::{Config, ZoneConfig};
pub use record::Record;

pub async fn entrypoint(state: State) -> Result<(), Error> {
    let zone_id = &state.config.zone.id;
    let records: CfListBody<Record> = state.get(&format!("/zones/{zone_id}/dns_records")).await?;
    if state.config.zone.restrictive_email {
        let _: CfNewRecordBody = state
            .post(
                &format!("/zones/{zone_id}/dns_records"),
                Record::restrictive_spf(),
            )
            .await?;
        let _: CfNewRecordBody = state
            .post(
                &format!("/zones/{zone_id}/dns_records"),
                Record::restrictive_dkim(),
            )
            .await?;
        let _: CfNewRecordBody = state
            .post(
                &format!("/zones/{zone_id}/dns_records"),
                Record::restrictive_dmarc(),
            )
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
