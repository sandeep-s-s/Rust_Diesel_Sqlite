use diesel::prelude::*;
use models::Post;
use std::env::args;
use rust_diesel_sqlite_demo::*;
fn main() {

    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(1))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Published post {}", post.title);
}