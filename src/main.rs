use puffy_blowhog_dns::State;
use reqwest::header::{HeaderMap, HeaderValue};

fn main() {
    dotenvy::dotenv().ok();
    let filename = std::env::args()
        .nth(1)
        .expect("This program expects exactly 1 argument");
    let api_token = std::env::var("CLOUDFLARE_API_TOKEN")
        .expect("Expected CLOUDFLARE_API_TOKEN in environment");
    let toml = std::fs::read_to_string(&filename)
        .unwrap_or_else(|e| panic!("Failed to open config file {filename}: {e}"));
    let config = toml::from_str(&toml).expect("Failed to deserialize toml");
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {api_token}")).unwrap(),
    );
    headers.append(
        "User-Agent",
        HeaderValue::from_static(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        )),
    );
    let http = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let state = State {
        config,
        api_token,
        http,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let output = rt.block_on(puffy_blowhog_dns::entrypoint(state));
    if let Err(e) = output {
        eprintln!("Error: {e}");
    }
}
