use serde::Deserialize;
use serde::Serialize;

use crate::CfResult;
use crate::Config;
use crate::Error;
use crate::Record;
use crate::State;
use crate::ZoneConfig;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneData {
    pub name: String,
}

pub async fn get(state: State, zone_id: String) -> Result<(), Error> {
    let zone_data: CfResult<ZoneData> = state.get(&format!("/zones/{zone_id}")).await?;
    let name = zone_data.result.name;
    let records_encapsulated: CfResult<Vec<Record>> =
        state.get(&format!("/zones/{zone_id}/dns_records")).await?;
    let records = records_encapsulated.result;
    let config = Config {
        zone: ZoneConfig {
            name,
            id: zone_id,
            restrictive_email: false,
        },
        records,
    };
    let toml_data = toml::to_string_pretty(&config)?;
    println!("{toml_data}");
    Ok(())
}
