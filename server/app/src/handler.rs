use axum_extra::extract::CookieJar;
use core::result::Result;

use crate::repository::Repository;
use axum::async_trait;
use openapi::{
    models::{
        PostLogin, ScheduleGroupIdGetPathParams, ScheduleGroupIdPostPathParams,
        ScheduleGroupIdPutPathParams, ScheduleItem,
    },
    Api, GroupPostResponse, LoginPostResponse, MeGetResponse, ScheduleGroupIdGetResponse,
    ScheduleGroupIdPostResponse, ScheduleGroupIdPutResponse, SignUpPostResponse,
};

#[async_trait]
impl Api for Repository {
    async fn sign_up_post(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: PostLogin,
    ) -> Result<SignUpPostResponse, String> {
        self.add_user(body.user_name.clone(), body.password).await?;

        let user_session = match self.create_session_for_user(body.user_name).await {
            Ok(session) => session,
            Err(e) => {
                println!("error: {}", e);
                "".to_string()
            }
        };

        let result = Ok(SignUpPostResponse::Status200_Success {
            set_cookie: Some(user_session),
        });

        println!("Login requested");

        result
    }

    async fn login_post(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: PostLogin,
    ) -> Result<LoginPostResponse, String> {
        let copied_password: String = body.password.clone();
        let db_result: Result<bool, String> = self.check_user(body.user_name, body.password).await;

        

        match db_result {
            Ok(true) => Ok(LoginPostResponse::Status200_Success {
                set_cookie: (Some("".to_string())),
            }),
            Ok(false) => {
                println!("password: {} is not correct", copied_password);
                Ok(LoginPostResponse::Status400_BadRequest)
            }
            Err(e) => {
                println!("{}", e);
                Ok(LoginPostResponse::Status400_BadRequest)
            }
        }
    }

    async fn me_get(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        cookies: CookieJar,
    ) -> Result<MeGetResponse, String> {
        let user_session = cookies.get("user_session").unwrap().value();

        let user_name = self
            .load_session_from_cookie(user_session)
            .await
            .unwrap()
            .unwrap();

        let db_result = self.get_groups_by_user(user_name).await?;

        Ok(MeGetResponse::Status200_Success(db_result))
    }

    async fn group_post(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        body: openapi::models::PostGroup,
    ) -> Result<GroupPostResponse, String> {
        let _db_result = self.create_group(body.group_name).await;

        Ok(GroupPostResponse::Status200_Success)
    }

    async fn schedule_group_id_get(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        path_params: ScheduleGroupIdGetPathParams,
    ) -> Result<ScheduleGroupIdGetResponse, String> {
        let aaa = "Stringにしたいんだが！？".to_string();
        let bbb: Vec<ScheduleItem> = vec![];

        let result = match path_params.group_id.as_str() {
            "_aaa" => Ok(ScheduleGroupIdGetResponse::Status200_Success(bbb)),
            _ => Err(aaa),
        };

        result
    }

    async fn schedule_group_id_post(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: ScheduleGroupIdPostPathParams,
        body: ScheduleItem,
    ) -> Result<ScheduleGroupIdPostResponse, String> {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(ScheduleGroupIdPostResponse::Status200_Success),
            _ => Err(aaa),
        };

        result
    }

    async fn schedule_group_id_put(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: CookieJar,
        _path_params: ScheduleGroupIdPutPathParams,
        body: ScheduleItem,
    ) -> Result<ScheduleGroupIdPutResponse, String> {
        let aaa = "Stringにしたいんだが！？".to_string();

        let result = match body.user_name.as_str() {
            "_aaa" => Ok(ScheduleGroupIdPutResponse::Status200_Success),
            _ => Err(aaa),
        };

        result
    }
}
