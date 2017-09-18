use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::post::*;
use std::collections::HashMap;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;

use util::auth::User;
use util::response::ResponseEnum;

#[get("/admin/post")]
pub fn get_posts(_user: User, db: DB) -> Template {
    let result = Post::query_all(db.conn());
    let mut context = HashMap::new();
    context.insert("posts", result);
    Template::render("admin/post_list", &context)
}

#[post("/admin/post",data="<new_post>")]
pub fn add_post(db: DB, new_post: Json<NewPost>) -> Json<ResponseEnum> {
    if NewPost::insert(&new_post.0, db.conn()) {
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}

#[get("/admin/new_post")]
pub fn add_post_page(_user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("foo", "bar");
    Template::render("admin/post", &context)
}

#[get("/admin/<id>")]
pub fn edit_post(id: i32, db: DB) -> Template {
    let result = Post::query_by_id(db.conn(), id);
    let mut context = HashMap::new();
    context.insert("post", result.first());
    Template::render("admin/post", &context)
}
#[delete("/admin/post/<id>")]
pub fn delete_post(id: i32, db: DB) -> Json<ResponseEnum> {
    if Post::delete_with_id(db.conn(), id) {
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}
#[put("/admin/post",data="<update_post>")]
pub fn update_post(update_post: Json<Post>, db: DB) -> Json<ResponseEnum> {
    println!("Call update");
    if Post::update(db.conn(), &update_post.0) {
        Json(ResponseEnum::SUCCESS)
    } else {
        Json(ResponseEnum::ERROR)
    }
}
