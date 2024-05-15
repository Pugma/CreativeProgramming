use axum_extra::extract::CookieJar;
use core;
use openapi::Api;

#[derive(Clone)]
pub struct Count(pub i64);

impl AsRef<Count> for Count {
    fn as_ref(&self) -> &Count {
        self
    }
}

impl Api for Count {
    fn login_post<'life0, 'async_trait>(
        &'life0 self,
        method: axum::http::Method,
        host: axum::extract::Host,
        cookies: CookieJar,
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

        let result = match body.group_code {
            aaa => Ok(openapi::LoginPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: axum::http::Method,
        host: axum::extract::Host,
        cookies: CookieJar,
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

        let result = match path_params.group_id {
            aaa => Ok(openapi::ScheduleGroupIdGetResponse::Status200_Success(bbb)),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_post<'life0, 'async_trait>(
        &'life0 self,
        method: axum::http::Method,
        host: axum::extract::Host,
        cookies: CookieJar,
        path_params: openapi::models::ScheduleGroupIdPostPathParams,
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

        let result = match body.user_name {
            aaa => Ok(openapi::ScheduleGroupIdPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }

    fn schedule_group_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: axum::http::Method,
        host: axum::extract::Host,
        cookies: CookieJar,
        path_params: openapi::models::ScheduleGroupIdPutPathParams,
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

        let result = match body.user_name {
            aaa => Ok(openapi::ScheduleGroupIdPutResponse::Status200_Success),
            _ => Err(aaa),
        };

        Box::pin(async { result })
    }
}
