struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  // let user1 = User {
  //   email: String::from("someone@example.com"),
  //   username: String::from("myusername"),
  //   active: true,
  //   sign_in_count: 1,
  // };
  
  // With builder method
  let user1 = build_user(
    String::from("someone@example.com"),
    String::from("myusername"),
  );
  println!("Hello, {}!", user1.username);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
