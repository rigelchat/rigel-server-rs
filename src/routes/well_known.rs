use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/spacebar/client", get(get_spacebar_config));
}

async fn get_spacebar_config() -> Json<ClientResponse> {
    return Json(ClientResponse {
        api: SpacebarClientApi {
            base_url: env::var("PUBLIC_HTTP_URL").unwrap_or_default(),
            api_versions: ApiVersions {
                default: 0, // todo: move into a constant
                active: vec![0]
            }
        },
        cdn: SpacebarClientCdn {
            base_url: format!("{}/cdn", env::var("PUBLIC_HTTP_URL").unwrap_or_default())
        },
        gateway: SpacebarClientGateway {
            base_url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default()),
            encoding: vec![
                GatewayEncoding::Json
            ],
            compression: vec![
                Some(GatewayCompression::ZstdStream),
                None
            ]
        }
    });
}

#[derive(Serialize)]
struct ApiVersions {
    default: u8,
    active: Vec<u8>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpacebarClientApi {
    base_url: String,
    api_versions: ApiVersions
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpacebarClientCdn {
    base_url: String
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
enum GatewayEncoding {
    Json
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
enum GatewayCompression {
    ZstdStream
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpacebarClientGateway {
    base_url: String,
    encoding: Vec<GatewayEncoding>,
    compression: Vec<Option<GatewayCompression>>
}

#[derive(Serialize)]
struct ClientResponse {
    api: SpacebarClientApi,
    cdn: SpacebarClientCdn,
    gateway: SpacebarClientGateway
}