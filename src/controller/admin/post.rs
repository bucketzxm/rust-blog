use diesel::result::QueryResult;
use diesel; //
use diesel::prelude::*;
use dal::diesel_pool::DB;
use dal::models::post::*;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;
use chrono::prelude::*;
use rocket_contrib::Json;

use util::auth::User;

#[get("/admin/post")]
pub fn get_posts(_user: User, db: DB) -> Template {
    let result = Post::query_all(db.conn());
    let mut context = HashMap::new();
    context.insert("posts", result);
    Template::render("admin/post_list", &context)
}

#[post("/admin/post",data="<new_post>")]
pub fn add_post(db: DB, new_post: Json<NewPost>) -> &'static str {
    if NewPost::insert(&new_post.0, db.conn()) {
        "success"
    } else {
        "error"
    }
}

#[get("/admin/new_post")]
pub fn add_post_page() -> Template {
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
pub fn delete_post(id: i32, db: DB) -> &'static str {
    if Post::delete_with_id(db.conn(), id) {
        "success"
    } else {
        "error"
    }
}
#[put("/admin/post",data="<update_post>")]
pub fn update_post(update_post: Json<Post>, db: DB) -> &'static str {
    println!("Call update");
    // println!("{:?}", &update_post.0);
    if Post::update_post(db.conn(), &update_post.0) {
        "success"
    } else {
        "error"
    }
}
