/// #[async_trait] is no longer needed.
/// Axum0.8: https://github.com/tokio-rs/axum/blob/e0b55d750390d810028caad0387058751611c1b4/axum-core/CHANGELOG.md?plain=1#L25
use crate::errors::AuthenticateError;
use crate::errors::Error;
use crate::settings::SETTINGS;
use crate::utils::token;
use crate::utils::token::TokenUser;

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

// #[async_trait]
impl<S> FromRequestParts<S> for TokenUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let secret = SETTINGS.auth.secret.as_str();
        let token_data =
            token::decode(bearer.token(), secret).map_err(|_| AuthenticateError::InvalidToken)?;

        Ok(token_data.claims.user)
    }
}
