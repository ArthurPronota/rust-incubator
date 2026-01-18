#[derive(Clone)]
struct Post {
    id: u64,
    user_id: u64,
    title: String,
    body: String,
}

fn repost(post: &Post, new_author_id: u64) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Oops!
    new_post
}

fn main() {
    println!("Implement me!");
}
