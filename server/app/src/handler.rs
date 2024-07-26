use crate::repository::Repository;
use axum::http::Method;
use axum::{
    body::Body,
    extract::{Host, Path, State},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use openapi::models::PostGroup;
use openapi::{
    models::{PostLogin, ScheduleGroupIdGetPathParams, ScheduleItem},
    LoginPostResponse, ScheduleGroupIdPostResponse, ScheduleGroupIdPutResponse,
};

use crate::errors::Result;

// #[debug_handler]
pub async fn sign_up_post(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    State(repo): State<Repository>,
    Json(body): Json<PostLogin>,
) -> Result<Response> {
    repo.add_user(body.user_name.clone(), body.password.clone())
        .await?;

    let _user_session = match repo.create_session_for_user(body.user_name.clone()).await {
        Ok(session) => session,
        Err(e) => {
            println!("error: {}", e);
            "".to_string()
        }
    };

    println!("Login requested");

    let response = Response::builder();

    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn login_post(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    State(repo): State<Repository>,
    Json(body): Json<PostLogin>,
) -> Result<Response> {
    let copied_password: String = body.password.clone();
    let db_result: Result<bool, String> = repo.check_user(body.user_name, body.password).await;

    let _result = match db_result {
        Ok(true) => Ok(LoginPostResponse::Status200_Success),
        Ok(false) => {
            println!("password: {} is not correct", copied_password);
            Ok(LoginPostResponse::Status400_BadRequest)
        }
        Err(e) => Err(e),
    };

    let response = Response::builder();

    // let resp = match result {
    //     Ok(rsp) => match rsp {
    //         LoginPostResponse::Status200_Success => {
    //             if let Some(set_cookie) = set_cookie {
    //                 let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
    //                     Ok(val) => val,
    //                     Err(e) => {
    //                         return Response::builder()
    //                                                                 .status(StatusCode::INTERNAL_SERVER_ERROR)
    //                                                                 .body(Body::from(format!("An internal server error occurred handling set_cookie header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
    //                     }
    //                 };

    //                 {
    //                     let mut response_headers = response.headers_mut().unwrap();
    //                     response_headers.insert(HeaderName::from_static(""), set_cookie);
    //                 }
    //             }

    //             let mut response = response.status(200);
    //             response.body(Body::empty())
    //         }
    //         LoginPostResponse::Status400_BadRequest => {
    //             let mut response = response.status(400);
    //             response.body(Body::empty())
    //         }
    //     },
    //     Err(_) => {
    //         // Application code returned an error. This should not happen, as the implementation should
    //         // return a valid response.
    //         response.status(500).body(Body::empty())
    //     }
    // };

    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn me_get(
    _method: Method,
    _host: Host,
    cookies: CookieJar,
    State(repo): State<Repository>,
    Json(_body): Json<PostLogin>,
) -> crate::Result<Response> {
    let user_session = cookies.get("user_session").unwrap().value();

    let user_name = repo
        .load_session_from_cookie(user_session)
        .await
        .unwrap()
        .unwrap();

    let _db_result = repo.get_groups_by_user(user_name).await.unwrap();

    let response = Response::builder();
    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn group_post(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    State(repo): State<Repository>,
    Json(body): Json<PostGroup>,
) -> crate::Result<Response> {
    let _db_result = repo.create_group(body.group_name).await;

    let response = Response::builder();
    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn schedule_group_id_get(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    Path(_path_params): Path<ScheduleGroupIdGetPathParams>,
    State(_repo): State<Repository>,
    // Json(body): Json<Sched>,
) -> crate::Result<Response> {
    // let user_name: UserName = UserName("".to_string());
    let _aaa = "Stringにしたいんだが！？".to_string();
    let _bbb: Vec<ScheduleItem> = vec![];

    // let a = _repo.(_path_params.group_id).await.unwrap();

    // let result = match path_params.group_id.as_str() {
    //     "_aaa" => Ok(ScheduleGroupIdGetResponse::Status200_Success(bbb)),
    //     _ => Err(aaa),
    // };

    let response = Response::builder();
    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn schedule_group_id_post(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    State(_repo): State<Repository>,
    Json(body): Json<PostLogin>,
) -> crate::Result<Response> {
    let aaa = "Stringにしたいんだが！？".to_string();

    let _result = match body.user_name.as_str() {
        "_aaa" => Ok(ScheduleGroupIdPostResponse::Status200_Success),
        _ => Err(aaa),
    };

    let response = Response::builder();
    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}

pub async fn schedule_group_id_put(
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    State(_repo): State<Repository>,
    Json(body): Json<PostLogin>,
) -> crate::Result<Response> {
    let aaa = "Stringにしたいんだが！？".to_string();

    let _result = match body.user_name.as_str() {
        "_aaa" => Ok(ScheduleGroupIdPutResponse::Status200_Success),
        _ => Err(aaa),
    };

    let response = Response::builder();
    let rep = response.status(200).body(Body::empty()).unwrap();
    Ok(rep)
}
