use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
struct ApiVersions {
    default: u8,
    active: Vec<u8>
}

#[derive(Serialize)]
struct ClientApi {
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "apiVersions")]
    api_versions: ApiVersions
}

#[derive(Serialize)]
struct ClientCdn {
    #[serde(rename = "baseUrl")]
    base_url: String
}

#[derive(Serialize)]
enum GatewayEncoding {
    #[serde(rename = "json")]
    Json
}

#[derive(Serialize)]
enum GatewayCompression {
    #[serde(rename = "zstd-stream")]
    ZstdStream
}

#[derive(Serialize)]
struct ClientGateway {
    base_url: String,
    encoding: Vec<GatewayEncoding>,
    compression: Vec<Option<GatewayCompression>>
}

#[derive(Serialize)]
struct ClientResponse {
    api: ClientApi,
    cdn: ClientCdn,
    gateway: ClientGateway
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/spacebar/client", get(get_client))
}

async fn get_client() -> Json<ClientResponse> {
    return Json(ClientResponse {
        api: ClientApi {
            base_url: env::var("PUBLIC_HTTP_URL").unwrap_or_default(),
            api_versions: ApiVersions {
                default: 0, // todo: move into a constant
                active: vec![0]
            }
        },
        cdn: ClientCdn {
            base_url: format!("{}/cdn", env::var("PUBLIC_HTTP_URL").unwrap_or_default())
        },
        gateway: ClientGateway {
            base_url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default()),
            encoding: vec![
                GatewayEncoding::Json
            ],
            compression: vec![
                None
            ]
        },
    });
}