use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("Blah");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Blah", post.content());
}
