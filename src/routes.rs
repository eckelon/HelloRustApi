use crate::dao::people::get_people;
use warp::reply::{Html, Json};
use warp::{path, reply, Filter, Rejection, Reply};


fn get_people_route() -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(path("people"))
        .map(move || reply::json(&get_people()))
}

fn get_misco_route() -> impl Filter<Extract = (Html<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(path("hello"))
        .map(move || reply::html(String::from("Hello world!")))
}

pub fn get_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::any().and(get_people_route()).or(get_misco_route())
}
