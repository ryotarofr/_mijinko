/// Original author: Christian Gill (@gillchristian)
/// From: https://gist.github.com/gillchristian/db76e712cc02bff620b86f0cd2bfb691

/// #[async_trait] is no longer needed.
/// Axum0.8: https://github.com/tokio-rs/axum/blob/e0b55d750390d810028caad0387058751611c1b4/axum-core/CHANGELOG.md?plain=1#L25
// use async_trait::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::{request::Parts, StatusCode};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Limit {
    limit: u32,
}

impl Default for Limit {
    fn default() -> Self {
        Self { limit: 100 }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Offset {
    offset: u64,
}

#[derive(Debug, Clone)]
pub struct Pagination {
    /// The number of documents to skip before counting.
    pub offset: u64,
    /// The maximum number of documents to query.
    pub limit: u32,
}

// #[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(Limit { limit }) = Query::<Limit>::from_request_parts(parts, state)
            .await
            .unwrap_or_default();

        let Query(Offset { offset }) = Query::<Offset>::from_request_parts(parts, state)
            .await
            .unwrap_or_default();

        Ok(Self { limit, offset })
    }
}
