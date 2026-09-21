use anyhow::Context;
use integrations::Origin;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub identity_url: Origin,
    pub payments_url: Origin,
    pub bind_addr: SocketAddr,
    pub upstream_timeout: Duration,
}

impl Config {
    /// `IDENTITY_URL` and `PAYMENTS_URL` are required origins. `BIND_ADDR`
    /// defaults to `127.0.0.1:8080`, `UPSTREAM_TIMEOUT_MS` to 2000.
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let origin = |key: &str| -> anyhow::Result<Origin> {
            let raw = get(key).with_context(|| format!("{key} is required"))?;
            Origin::parse(&raw).map_err(|error| anyhow::anyhow!("{key}: {error}"))
        };
        let timeout_ms: u64 = get("UPSTREAM_TIMEOUT_MS")
            .map(|raw| raw.parse())
            .transpose()
            .context("UPSTREAM_TIMEOUT_MS must be an integer")?
            .unwrap_or(2000);
        Ok(Self {
            identity_url: origin("IDENTITY_URL")?,
            payments_url: origin("PAYMENTS_URL")?,
            bind_addr: get("BIND_ADDR")
                .unwrap_or_else(|| "127.0.0.1:8080".into())
                .parse()
                .context("BIND_ADDR must be host:port")?,
            upstream_timeout: Duration::from_millis(timeout_ms),
        })
    }
}
