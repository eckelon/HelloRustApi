use crate::dao::people::get_people;
use nanoserde::SerJson;
use warp::{http, path, reply, Filter, Rejection, Reply};

fn get_people_route(
) -> impl Filter<Extract = (Result<http::Response<String>, http::Error>,), Error = Rejection> + Clone
{
    warp::get()
        .and(path("people"))
        .map(|| http::Response::builder().body(SerJson::serialize_json(&get_people())))
}

fn get_misco_route() -> impl Filter<Extract = (reply::Html<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(path("hello"))
        .map(|| reply::html(String::from("Hello world!")))
}

pub fn get_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::any().and(get_people_route()).or(get_misco_route())
}
