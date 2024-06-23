use diesel::prelude::*;
use models::Post;
use rust_diesel_sqlite_demo::*;
use std::env::args;
use rust_diesel_sqlite_demo::schema::posts::dsl::*;
fn main() {

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occurred while fetching post {}", post_id),
    }
}