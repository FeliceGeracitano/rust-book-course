use url::Url;

/// A provider base URL: scheme and host only. Paths are added per call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin(Url);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OriginError {
    #[error("not a valid URL: {0}")]
    Invalid(String),
    #[error("scheme must be http or https")]
    Scheme,
    #[error("origin must not contain a path, query, fragment, or credentials")]
    Extra,
}

impl Origin {
    pub fn parse(raw: &str) -> Result<Self, OriginError> {
        let url = Url::parse(raw).map_err(|error| OriginError::Invalid(error.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(OriginError::Scheme);
        }
        let bare_path = url.path().is_empty() || url.path() == "/";
        if !bare_path
            || url.query().is_some()
            || url.fragment().is_some()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(OriginError::Extra);
        }
        Ok(Self(url))
    }

    /// `path` starts with `/` and may carry a query string.
    pub fn join(&self, path: &str) -> Url {
        self.0
            .join(path)
            .expect("an http origin joined with an absolute path")
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
