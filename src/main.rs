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
    let config = match toml::from_str(&toml) {
        Ok(v) => v,
        Err(e) => {
            let mut line_number = 1;
            let mut col = 0;
            eprint!("{} at ", e.message());
            if let Some(err_loc) = e.span() {
                for (idx, char) in toml.chars().enumerate() {
                    col += 1;
                    if char == '\n' {
                        line_number += 1;
                        col = 0;
                    }
                    if err_loc.contains(&idx) {
                        break;
                    }
                }
                eprintln!("file {filename}:{line_number}:{col}");
                for (line_idx_0, line_str) in toml.lines().enumerate() {
                    let line_idx = line_idx_0 + 1;
                    if line_idx == line_number {
                        eprint!("{line_idx}: ");
                        for (col_idx_0, char) in line_str.chars().enumerate() {
                            let col_idx = col_idx_0 + 1;
                            if col_idx == col {
                                eprint!("\u{001b}[1;31m\u{001b}[4m{char}\u{001b}[24m\u{001b}[1;0m");
                            } else {
                                eprint!("{char}");
                            }
                        }
                        eprintln!();
                    } else if line_idx.abs_diff(line_number) <= 2 {
                        eprintln!("{line_idx}: {line_str}");
                    }
                }
            } else {
                eprintln!();
            }
            std::process::exit(1);
        }
    };
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
