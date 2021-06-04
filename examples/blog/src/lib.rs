pub struct Post {
  state: Option<Box<dyn State>>,
  content: String
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new()
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(self.state.as_ref().unwrap().add_text(text));
  }

  pub fn content(&self) -> &str {
    // Both are the same
    // (&self.state).unwrap().content(self)
    self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(& mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post:&'a Post) -> &'a str {
    ""
  }
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn add_text<'a>(&self, text: &'a str) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {approved_count: 0})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn add_text<'a>(&self, text: &'a str) -> &'a str {
    text
  }
}

struct PendingReview {approved_count: i8,}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(mut self: Box<Self>) -> Box<dyn State> {
    self.approved_count+= 1;
    if self.approved_count > 1 {Box::new(Published{})} else {self}
  }
  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft{})
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
  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post:&'a Post) -> &'a str {
    &post.content
  }
}