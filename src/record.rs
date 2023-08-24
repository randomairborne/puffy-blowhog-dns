#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BaseRecord {
    pub name: String,
    pub content: String,
    #[serde(default = "return_1_u32")]
    pub ttl: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default = "return_false", skip_serializing)]
    pub ignored: bool,
}

fn return_1_u32() -> u32 {
    1
}

fn return_false() -> bool {
    false
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Record {
    A {
        #[serde(flatten)]
        base: BaseRecord,
        proxied: bool,
    },
    Aaaa {
        #[serde(flatten)]
        base: BaseRecord,
        proxied: bool,
    },
    Caa,
    Cert,
    Cname,
    DnsKey,
    Ds,
    Https,
    Loc,
    Mx,
    Naptr,
    Ns,
    Ptr,
    Smimea,
    Srv,
    Sshfp,
    Svcb,
    Tlsa,
    Txt {
        #[serde(flatten)]
        base: BaseRecord,
    },
    Uri,
}

impl Record {
    pub fn restrictive_spf() -> Self {
        Self::Txt {
            base: BaseRecord {
                name: "@".to_string(),
                content: "v=spf1 -all".to_string(),
                ttl: 1, // auto
                comment: Some("SPF record automagically added by puffy blowhog".to_string()),
                tags: None,
                ignored: false,
            },
        }
    }
    pub fn restrictive_dkim() -> Self {
        Self::Txt {
            base: BaseRecord {
                name: "*._domainkey".to_string(),
                content: "v=DKIM1; p=".to_string(),
                ttl: 1, // auto
                comment: Some("DKIM record automagically added by puffy blowhog".to_string()),
                tags: None,
                ignored: false,
            },
        }
    }
    pub fn restrictive_dmarc() -> Self {
        Self::Txt {
            base: BaseRecord {
                name: "_dmarc".to_string(),
                content: "v=DMARC1; p=reject; sp=reject; adkim=s; aspf=s;".to_string(),
                ttl: 1, // auto
                comment: Some("DMARC record automagically added by puffy blowhog".to_string()),
                tags: None,
                ignored: false,
            },
        }
    }
}
