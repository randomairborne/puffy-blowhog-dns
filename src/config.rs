use crate::record::Record;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub zone: ZoneConfig,
    pub records: Vec<Record>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ZoneConfig {
    pub name: String,
    pub id: String,
    #[serde(default = "return_false")]
    pub restrictive_email: bool,
}

fn return_false() -> bool {
    false
}
