use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Record {
    A {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        proxied: bool,
    },
    Aaaa {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        proxied: bool,
    },
    Caa {
        name: String,
        data: CaaData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Cert {
        name: String,
        data: CertData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Cname {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        proxied: bool,
    },
    DnsKey {
        name: String,
        data: DnsKeyData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Ds {
        name: String,
        data: DsData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Https {
        name: String,
        data: HttpsData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Loc {
        name: String,
        data: LocData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Mx {
        name: String,
        content: String,
        priority: u16,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Naptr {
        name: String,
        data: NaptrData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Ns {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Ptr {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Smimea {
        name: String,
        data: SmimeaData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Srv {
        name: String,
        data: SrvData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Sshfp {
        name: String,
        data: SshfpData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Svcb {
        name: String,
        data: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Tlsa {
        name: String,
        data: TlsaData,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Txt {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
    },
    Uri {
        name: String,
        content: String,
        #[serde(default = "return_1_u32")]
        ttl: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        priority: u16
    },
}

fn return_1_u32() -> u32 {
    1
}

impl Record {
    pub fn restrictive_spf() -> Self {
        Self::Txt {
            name: "@".to_string(),
            content: "v=spf1 -all".to_string(),
            ttl: 1, // auto
            comment: Some("SPF record automagically added by puffy blowhog".to_string()),
            tags: None,
        }
    }
    pub fn restrictive_dkim() -> Self {
        Self::Txt {
            name: "*._domainkey".to_string(),
            content: "v=DKIM1; p=".to_string(),
            ttl: 1, // auto
            comment: Some("DKIM record automagically added by puffy blowhog".to_string()),
            tags: None,
        }
    }
    pub fn restrictive_dmarc() -> Self {
        Self::Txt {
            name: "_dmarc".to_string(),
            content: "v=DMARC1; p=reject; sp=reject; adkim=s; aspf=s;".to_string(),
            ttl: 1, // auto
            comment: Some("DMARC record automagically added by puffy blowhog".to_string()),
            tags: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CaaData {
    flags: u8,
    tag: String,
    value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CertData {
    algorithm: u8,
    certificate: String,
    key_tag: u16,
    #[serde(rename = "type")]
    kind: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DnsKeyData {
    algorithm: u8,
    flags: u16,
    protocol: u8,
    public_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DsData {
    algorithm: u8,
    digest: String,
    digest_type: u8,
    key_tag: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HttpsData {
    priority: u16,
    target: String,
    value: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocData {
    altitude:i32,
    lat_degrees: u8,
    lat_direction: LatitudeDirection,
    lat_minutes: Option<u8>,
    lat_seconds: Option<u8>,
    long_degrees: u8,
    long_direction: LongitudeDirection,
    long_minutes: Option<u8>,
    long_seconds: Option<u8>,
    precision_horz: Option<u32>,
    precision_vert: Option<u32>,
    size: Option<u32>
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatitudeDirection {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "S")]
    South
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LongitudeDirection {
    #[serde(rename = "E")]
    East,
    #[serde(rename = "W")]
    West
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NaptrData {
    flags: String,
    order: u16,
    preference: u16,
    regex: String,
    replacement: String,
    service: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SmimeaData {
    certificate: String,
    matching_type: u8,
    selector: u8,
    usage: u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SrvData {
    name: String,
    port: u16,
    priority: u16,
    proto: String,
    service: String,
    target: String,
    weight: u16
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SshfpData {
    algorithm: u8,
    fingerprint: String,
    #[serde(rename = "type")]
    kind: u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SvcbData {
    priority: u16,
    target: String,
    value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TlsaData {
    certificate: String,
    matching_type: u8,
    selector: u8,
    usage: u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UriData {
    content: String,
    weight: u16
}