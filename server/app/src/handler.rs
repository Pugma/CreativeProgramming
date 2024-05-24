use axum_extra::extract::CookieJar;
use core;
// use openapi::Api;
use sqlx::{mysql::MySqlQueryResult, MySql, Pool};

use crate::db;
use crate::openapi::Api;

#[derive(Clone)]
pub struct Count(pub Pool<MySql>);

impl AsRef<Count> for Count {
    fn as_ref(&self) -> &Count {
        self
    }
}

impl Api for Count {
    fn sign_up_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: openapi::models::PostLogin,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<openapi::SignUpPostResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let db_result: Result<MySqlQueryResult, String> = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                db::add_user(self.0.clone(), body.user_name, body.password).await
            })
        });

        let result = match db_result {
            Ok(_) => Ok(openapi::SignUpPostResponse::Status200_Success),
            _ => Err("Failed to ".to_string()),
        };

        println!("Login requested");

        Box::pin(async { result })
    }
    fn login_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: openapi::models::PostLogin,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<openapi::LoginPostResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "文字列リテラル".to_string();

        let result = match body.user_name.as_str() {
            "aaa" => Ok(openapi::LoginPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_get<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        path_params: openapi::models::ScheduleGroupIdGetPathParams,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<openapi::ScheduleGroupIdGetResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();
        let bbb: Vec<openapi::models::ScheduleItem> = vec![];

        let result = match path_params.group_id.as_str() {
            "_aaa" => Ok(openapi::ScheduleGroupIdGetResponse::Status200_Success(bbb)),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: openapi::models::ScheduleGroupIdPostPathParams,
        body: openapi::models::ScheduleItem,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<openapi::ScheduleGroupIdPostResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(openapi::ScheduleGroupIdPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_put<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: openapi::models::ScheduleGroupIdPutPathParams,
        body: openapi::models::ScheduleItem,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<openapi::ScheduleGroupIdPutResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(openapi::ScheduleGroupIdPutResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }
}
