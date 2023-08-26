use serde::{Deserialize, Serialize};

macro_rules! with_default_fields {
    (
        $StructName:ident$(<$($tparam:ident),*>)? { $($manual_fields:tt)* }
    ) => {
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
        // black magic to support optional generic params
        pub struct $StructName$(<$($tparam),*>)? {
            pub name: String,
            #[serde(default = "return_1_u32")]
            pub ttl: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub comment: Option<String>,
            #[serde(default = "empty_vec_string", skip_serializing_if = "Vec::is_empty")]
            pub tags: Vec<String>,
            $($manual_fields)*
        }

    }
}

with_default_fields!(CommonFieldsContent { pub content: String });
with_default_fields!(CommonFieldsContentAndPriority { pub content: String, pub priority: u16 });
with_default_fields!(CommonFieldsContentAndProxied { pub content: String, pub proxied: bool });
with_default_fields!(CommonFieldsData<T> { pub data: T });
with_default_fields!(CommonFieldsDataAndPriority<T> { pub data: T, pub priority: u16 });

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Record {
    A(CommonFieldsContentAndProxied),
    Aaaa(CommonFieldsContentAndProxied),
    Caa(CommonFieldsData<CaaData>),
    Cert(CommonFieldsData<CertData>),
    Cname(CommonFieldsContentAndProxied),
    DnsKey(CommonFieldsData<DnsKeyData>),
    Ds(CommonFieldsData<DsData>),
    Https(CommonFieldsData<HttpsData>),
    Loc(CommonFieldsData<LocData>),
    Mx(CommonFieldsContentAndPriority),
    Naptr(CommonFieldsData<NaptrData>),
    Ns(CommonFieldsContent),
    Ptr(CommonFieldsContent),
    Smimea(CommonFieldsData<SmimeaData>),
    Srv(CommonFieldsData<SrvData>),
    Sshfp(CommonFieldsData<SshfpData>),
    Svcb(CommonFieldsData<SvcbData>),
    Tlsa(CommonFieldsData<TlsaData>),
    Txt(CommonFieldsContent),
    Url(CommonFieldsDataAndPriority<UriData>),
}

fn return_1_u32() -> u32 {
    1
}

fn empty_vec_string() -> Vec<String> {
    Vec::new()
}

impl Record {
    pub fn restrictive_spf() -> Self {
        Self::Txt(CommonFieldsContent {
            ttl: 1, // auto
            comment: Some("SPF record automagically added by puffy blowhog".to_string()),
            tags: vec![],
            name: "@".to_string(),
            content: "v=spf1 -all".to_string(),
        })
    }
    pub fn restrictive_dkim() -> Self {
        Self::Txt(CommonFieldsContent {
            ttl: 1, // auto
            comment: Some("DKIM record automagically added by puffy blowhog".to_string()),
            tags: vec![],
            name: "*._domainkey".to_string(),
            content: "v=DKIM1; p=".to_string(),
        })
    }
    pub fn restrictive_dmarc() -> Self {
        Self::Txt(CommonFieldsContent {
            ttl: 1, // auto
            comment: Some("DMARC record automagically added by puffy blowhog".to_string()),
            tags: vec![],
            name: "_dmarc".to_string(),
            content: "v=DMARC1; p=reject; sp=reject; adkim=s; aspf=s;".to_string(),
        })
    }
    pub fn normalize_name(self, zone_name: &str) -> Self {
        match self {
            Self::A(rec) => Self::A(rec.normalize_name(zone_name)),
            Self::Aaaa(rec) => Self::Aaaa(rec.normalize_name(zone_name)),
            Self::Caa(rec) => Self::Caa(rec.normalize_name(zone_name)),
            Self::Cert(rec) => Self::Cert(rec.normalize_name(zone_name)),
            Self::Cname(rec) => Self::Cname(rec.normalize_name(zone_name)),
            Self::DnsKey(rec) => Self::DnsKey(rec.normalize_name(zone_name)),
            Self::Ds(rec) => Self::Ds(rec.normalize_name(zone_name)),
            Self::Https(rec) => Self::Https(rec.normalize_name(zone_name)),
            Self::Loc(rec) => Self::Loc(rec.normalize_name(zone_name)),
            Self::Mx(rec) => Self::Mx(rec.normalize_name(zone_name)),
            Self::Naptr(rec) => Self::Naptr(rec.normalize_name(zone_name)),
            Self::Ns(rec) => Self::Ns(rec.normalize_name(zone_name)),
            Self::Ptr(rec) => Self::Ptr(rec.normalize_name(zone_name)),
            Self::Smimea(rec) => Self::Smimea(rec.normalize_name(zone_name)),
            Self::Srv(rec) => Self::Srv(rec.normalize_name(zone_name)),
            Self::Sshfp(rec) => Self::Sshfp(rec.normalize_name(zone_name)),
            Self::Svcb(rec) => Self::Svcb(rec.normalize_name(zone_name)),
            Self::Tlsa(rec) => Self::Tlsa(rec.normalize_name(zone_name)),
            Self::Txt(rec) => Self::Txt(rec.normalize_name(zone_name)),
            Self::Url(rec) => Self::Url(rec.normalize_name(zone_name)),
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
    altitude: i32,
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
    size: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatitudeDirection {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "S")]
    South,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LongitudeDirection {
    #[serde(rename = "E")]
    East,
    #[serde(rename = "W")]
    West,
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
    usage: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SrvData {
    name: String,
    port: u16,
    priority: u16,
    proto: String,
    service: String,
    target: String,
    weight: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SshfpData {
    algorithm: u8,
    fingerprint: String,
    #[serde(rename = "type")]
    kind: u8,
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
    usage: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UriData {
    content: String,
    weight: u16,
}
