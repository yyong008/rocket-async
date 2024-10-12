use crate::db::Db;
use rocket::serde::{ json::Json};
use crate::schema::posts;
use diesel::prelude::*;
use rocket::response::{Debug};
use rocket_db_pools::{Connection};
use diesel_async::RunQueryDsl;
use rocket::http::Status;
use crate::models::{NewPost, Post};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
pub async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i32>>, rocket::http::Status> {
    // 使用 diesel_async 进行异步查询
    let ids_result = posts::table
        .select(posts::id)
        .load::<i32>(&mut db)
        .await;

    match ids_result {
        Ok(ids) => Ok(Json(ids)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[get("/all")]
pub async fn all(mut db: Connection<Db>) -> Result<Json<Vec<Post>>, rocket::http::Status> {
    use crate::schema::posts::dsl::*;
    // 使用 diesel_async 进行异步查询
    let res = posts
        .load::<Post>(&mut db)
        .await;

    match res {
        Ok(p) => Ok(Json(p)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[post("/", format="json", data="<new_post>")]
pub async fn create(mut db: Connection<Db>, new_post: Json<NewPost<'_>>) -> Result<Json<Post>, Status> {
    let post_data = new_post.into_inner();

    let news_post_result = diesel::insert_into(posts::table)
        .values(&post_data)
        .get_result::<Post>(&mut db)
        .await;

    match news_post_result {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[put("/<id>", format = "json", data = "<updated_post>")]
pub async fn update(mut db: Connection<Db>, id: i32, updated_post: Json<Post>) -> Result<Json<Post>, Status> {
    let updated_post_data = updated_post.into_inner();

    // 这里使用了 `id` 来查找数据库中的记录
    let update_result = diesel::update(posts::table.find(id))
        .set(&updated_post_data)
        .get_result::<Post>(&mut db)
        .await;

    match update_result {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[delete("/<id>")]
pub async fn delete(mut db: Connection<Db>, id: i32) -> Result<Status, Status> {
    let delete_result = diesel::delete(posts::table.find(id))
        .execute(& mut db)
        .await;

    match delete_result {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::InternalServerError)
    }
}