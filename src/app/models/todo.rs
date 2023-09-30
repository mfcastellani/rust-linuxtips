use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

// Implement `Display` for `Todo`.
// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_id = format!("{: >10}", self.id);
        if self.completed {
            write!(f, "{} [x] - {}", formatted_id, self.title)
        } else {
            write!(f, "{} [ ] - {}", formatted_id, self.title)
        }
    }
}