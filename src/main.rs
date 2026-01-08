use std::{fs::read_to_string, net::SocketAddr, sync::LazyLock};

use askama::Template;
use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use fjall::{Database, Keyspace, KeyspaceCreateOptions, KvSeparationOptions};
use serde::Deserialize;
use subtitles::Subtitle;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

static KS: LazyLock<Keyspace> = LazyLock::new(|| {
    let db = Database::builder(&*CONFIG.db).open().unwrap();
    db.keyspace("subtitles", || {
        KeyspaceCreateOptions::default().with_kv_separation(Some(KvSeparationOptions::default()))
    })
    .unwrap()
});

#[derive(Deserialize, Debug)]
struct Config {
    subtitle_dir: String,
    address: String,
    db: String,
}

static CONFIG: LazyLock<Config> = LazyLock::new(Config::load_config);

impl Config {
    fn load_config() -> Config {
        let cfg_file = std::env::args()
            .nth(1)
            .unwrap_or_else(|| "config.toml".to_owned());
        if let Ok(config_toml_content) = read_to_string(cfg_file) {
            let config: Config = basic_toml::from_str(&config_toml_content).unwrap();
            config
        } else {
            panic!("Config file not found");
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/style.css", get(style))
        .nest_service("/static/", ServeDir::new(&*CONFIG.subtitle_dir));
    let addr: SocketAddr = CONFIG.address.parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn home(Query(input): Query<QuerySearch>) -> impl IntoResponse {
    let search = input.search.unwrap_or_default().to_lowercase();
    println!("Searching for: {}", search);

    let mut found = vec![];
    let mut total = 0;
    let mut subtitles = vec![];
    if !search.is_empty() {
        for g in KS.iter() {
            let k = g.key().unwrap();
            let str = String::from_utf8_lossy(&k);
            let len = str.len();
            let str_n = &str[..len - 5];
            if str_n.contains(&search) {
                total += 1;
                found.push(k);
                if total >= 100 {
                    break;
                }
            }
        }

        for key in found {
            let v = KS.get(&key).unwrap().unwrap();
            let subtitle: Subtitle = serde_json::from_slice(&v).unwrap();
            subtitles.push(subtitle);
        }
    }
    let body = SearchPage {
        search,
        total,
        subtitles,
    };
    into_response(&body)
}

#[derive(Debug, Deserialize)]
struct QuerySearch {
    search: Option<String>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct SearchPage {
    search: String,
    subtitles: Vec<Subtitle>,
    total: usize,
}

fn into_response<T: Template>(t: &T) -> Response<Body> {
    match t.render() {
        Ok(body) => Html(body).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn style() -> impl IntoResponse {
    let headers = [
        (header::CONTENT_TYPE, "text/css"),
        (
            header::CACHE_CONTROL,
            "public, max-age=1209600, s-maxage=86400",
        ),
    ];

    (headers, include_str!("../style.css"))
}
