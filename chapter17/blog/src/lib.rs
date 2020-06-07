use std::cell::RefCell;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        } 
    }

    pub fn add_text(&mut self, text: &str) {
        let current_content = self.content.clone();
        self.content = String::new();
        self.content.push_str(self.state.as_ref().unwrap().add_text(&current_content, text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    
    pub fn request_review(&mut self) {
        // Use take to set state to None so that the value is obtained and not borrowed
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // Box<Self> so that it can only be called on the box
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, current_content: &'a str, new_text: &'a str) -> &'a str {
        current_content
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {approve_count: RefCell::new(0)})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, current_content: &'a str, new_text: &'a str) -> &'a str {
        new_text
    }
}

struct PendingReview {
    // Use Interior mutability pattern
    approve_count: RefCell<i32>,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.replace_with
        let old_count = self.approve_count.replace_with(|&mut old| old + 1);
        if old_count == 1 {
            return Box::new(Published {})
        }
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
