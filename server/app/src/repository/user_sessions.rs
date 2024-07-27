use std::time::Duration;

use super::constants::{SESSION_COOKIE_DURATION, SESSION_COOKIE_NAME};
use anyhow::{Context, Result};
use async_session::{Session, SessionStore};
use axum::http::{header::SET_COOKIE, HeaderMap};

use super::UserName;
use crate::Repository;

#[allow(unused)]
impl Repository {
    pub async fn create_session_for_user(&self, user: String) -> Result<String> {
        let mut session = Session::new();

        session
            .insert("user", user)
            .with_context(|| "failed to insert user into session")?;

        session.expire_in(Duration::from_secs(SESSION_COOKIE_DURATION));

        let res = self
            .session_store
            .store_session(session)
            .await
            .with_context(|| "failed to save session to database")?
            .with_context(|| "unexpected error while converting session to cookie")?;

        Ok(res)
    }

    pub async fn load_session_from_cookie(&self, cookie: &str) -> Result<Option<UserName>> {
        let session = self.session_store.load_session(cookie.to_string()).await?;
        Ok(session.and_then(|s| s.get("user")))
    }

    pub async fn destroy_session_for_cookie(&self, cookie: &str) -> Result<Option<()>> {
        let Some(session) = self.session_store.load_session(cookie.to_string()).await? else {
            return Ok(None);
        };
        self.session_store.destroy_session(session).await?;
        Ok(Some(()))
    }
}

pub trait Aaa {
    async fn make_cookie_header(value: String) -> crate::Result<HeaderMap>;
}

impl Aaa for HeaderMap {
    async fn make_cookie_header(value: String) -> crate::Result<HeaderMap> {
        let header = [(
            SET_COOKIE,
            format!("{SESSION_COOKIE_NAME}={value}; Path=/; Max-Age={SESSION_COOKIE_DURATION}; HttpOnly",)
                .parse()
                .with_context(|| "failed to set cookie to header value")?,
        )]
        .into_iter()
        .collect();
        Ok(header)
    }
}
