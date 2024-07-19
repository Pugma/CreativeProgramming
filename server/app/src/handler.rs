use axum_extra::extract::CookieJar;
use core::{future::Future, marker, pin};
use sqlx::mysql::MySqlQueryResult;

use crate::repository::Repository;
use axum::async_trait;
use openapi::{
    models::{
        GroupItem, PostLogin, ScheduleGroupIdGetPathParams, ScheduleGroupIdPostPathParams,
        ScheduleGroupIdPutPathParams, ScheduleItem,
    },
    Api, GroupPostResponse, LoginPostResponse, MeGetResponse, ScheduleGroupIdGetResponse,
    ScheduleGroupIdPostResponse, ScheduleGroupIdPutResponse, SignUpPostResponse,
};

#[async_trait]
impl Api for Repository {
    fn sign_up_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: PostLogin,
    ) -> pin::Pin<
        Box<dyn Future<Output = Result<SignUpPostResponse, String>> + marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let db_result: Result<MySqlQueryResult, String> = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.add_user(body.user_name, body.password).await })
        });

        let result = match db_result {
            core::result::Result::Ok(_) => Ok(SignUpPostResponse::Status200_Success),
            Err(a) => {
                println!("{}", a);
                Ok(SignUpPostResponse::Status400_BadRequest)
            }
        };

        println!("Login requested");

        Box::pin(async { result })
    }
    fn login_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: PostLogin,
    ) -> pin::Pin<
        Box<dyn Future<Output = Result<LoginPostResponse, String>> + marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let copied_password: String = body.password.clone();
        let db_result: Result<bool, String> = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.check_user(body.user_name, body.password).await })
        });

        let result = match db_result {
            Ok(true) => Ok(LoginPostResponse::Status200_Success),
            Ok(false) => {
                println!("password: {} is not correct", copied_password);
                Ok(LoginPostResponse::Status400_BadRequest)
            }
            Err(e) => {
                println!("{}", e);
                Ok(LoginPostResponse::Status400_BadRequest)
            }
        };

        Box::pin(async { result })
    }

    fn me_get<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
    ) -> pin::Pin<
        Box<dyn Future<Output = Result<MeGetResponse, String>> + marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let user_name: String = "".to_string();

        let db_result: Result<Vec<GroupItem>, String> = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.get_groups_by_user(user_name).await })
        });

        let result = match db_result {
            Ok(result) => Ok(MeGetResponse::Status200_Success(result)),
            Err(e) => Err(e),
        };

        Box::pin(async { result })
    }

    fn group_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: openapi::models::PostGroup,
    ) -> core::pin::Pin<
        Box<
            dyn Future<Output = Result<GroupPostResponse, String>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let _db_result: Result<(), sqlx::Error> = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.create_group(body.group_name).await })
        });

        Box::pin(async { Ok(GroupPostResponse::Status200_Success) })
    }

    fn schedule_group_id_get<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        path_params: ScheduleGroupIdGetPathParams,
    ) -> pin::Pin<
        Box<
            dyn Future<Output = Result<ScheduleGroupIdGetResponse, String>>
                + marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();
        let bbb: Vec<ScheduleItem> = vec![];

        let result = match path_params.group_id.as_str() {
            "_aaa" => Ok(ScheduleGroupIdGetResponse::Status200_Success(bbb)),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_post<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: ScheduleGroupIdPostPathParams,
        body: ScheduleItem,
    ) -> pin::Pin<
        Box<
            dyn Future<Output = Result<ScheduleGroupIdPostResponse, String>>
                + marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(ScheduleGroupIdPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_put<'life0, 'async_trait>(
        &'life0 self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: ScheduleGroupIdPutPathParams,
        body: ScheduleItem,
    ) -> pin::Pin<
        Box<
            dyn Future<Output = Result<ScheduleGroupIdPutResponse, String>>
                + marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(ScheduleGroupIdPutResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }
}
