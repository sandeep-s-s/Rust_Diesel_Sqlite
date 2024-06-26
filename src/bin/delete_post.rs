
use std::env::args;
use diesel::prelude::*;
use rust_diesel_sqlite_demo::establish_connection;
use rust_diesel_sqlite_demo::schema::posts::dsl::*;

fn main() {

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{target}%");

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {num_deleted} posts");
}