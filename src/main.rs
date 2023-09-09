#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, HeaderName, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{any, get},
};

#[allow(clippy::unused_async)]
async fn home(
    ConnectInfo(sock_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<HtmlOrRaw, Error> {
    let accept = headers
        .get("Accept")
        .map_or("*/*", |x| x.to_str().unwrap_or("invalid header value"));
    let ip = get_ip(sock_addr, &headers, state)?;
    if accept.contains("text/html") {
        Ok(HtmlOrRaw::Html(include_str!("index.html")))
    } else {
        Ok(HtmlOrRaw::Raw(format!("{ip}\n")))
    }
}

#[allow(clippy::unused_async)]
async fn raw(
    ConnectInfo(sock_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<String, Error> {
    let ip = get_ip(sock_addr, &headers, state)?;
    Ok(format!("{ip}\n"))
}

fn get_ip(addr: SocketAddr, headers: &HeaderMap, state: AppState) -> Result<IpAddr, Error> {
    if let Some(header_name) = state.header {
        if let Some(header) = headers.get(&*header_name) {
            let sock_str = header.to_str()?;
            Ok(IpAddr::from_str(sock_str)?)
        } else {
            Err(Error::NoHeader)
        }
    } else {
        Ok(addr.ip())
    }
}

async fn nocors<B: Send>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    response
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        std::env::var("PORT")
            .unwrap_or_else(|_| 8080.to_string())
            .parse::<u16>()
            .unwrap_or(8080),
    ));
    let client_ip_var = std::env::var("CLIENT_IP_HEADER").ok();
    let state = AppState::new(client_ip_var);
    let app = axum::Router::new()
        .route("/", get(home))
        .route("/raw", any(raw))
        .layer(axum::middleware::from_fn(nocors))
        .with_state(state.clone());
    let sd = tokio_shutdown::Shutdown::new().unwrap();
    println!("Listening on http://{addr} for ip requests");
    let sd_s = sd.clone();
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            sd_s.handle().await;
        })
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    header: Option<Arc<HeaderName>>,
}

impl AppState {
    #[must_use]
    /// # Panics
    /// This function can panic when its hardcoded values are invalid
    /// or the passed `client_ip_name` is not a valid header name
    pub fn new(client_ip_name: Option<String>) -> Self {
        Self {
            header: client_ip_name.map(|v| Arc::new(HeaderName::try_from(v).unwrap())),
        }
    }
}

pub enum HtmlOrRaw {
    Html(&'static str),
    Raw(String),
}

impl IntoResponse for HtmlOrRaw {
    fn into_response(self) -> Response {
        match self {
            Self::Html(v) => axum::response::Html(v).into_response(),
            Self::Raw(v) => v.into_response(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No header found")]
    NoHeader,
    #[error("Could not convert supplied header to string")]
    ToStr(#[from] axum::http::header::ToStrError),
    #[error("Could not convert supplied header to IP address")]
    ToAddr(#[from] std::net::AddrParseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
