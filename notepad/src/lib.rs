use std::fs::{self, File};
use std::io::{self, Read, Write};
// use super::notepad::Notepad;

pub mod notepad {

  pub struct Notepad {
    content: String,
  }

  impl Notepad {
    pub fn new() -> Self {
      Notepad {
        content: String::new(),
      }
    }

    pub fn load_from_file(&mut self, path: &str) -> io::Result<()> {
      let mut file = File::open(path)?;
      file.read_to_string(&mut self.content)?;
      Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
      let mut file = File::create(path)?;
      file.write_all(self.content.as_bytes())?;
      Ok(())
    }

    pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
    }

    pub fn get_content(&self) -> &str {
      &self.content
    }

    pub fn clear(&mut self) {
      self.content.clear();
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_add_text() {
    let mut notepad = Notepad::new();
    notepad.add_text("Hello, world!");
    assert_eq!(notepad.get_content(), "Hello, world!");
  }

  #[test]
  fn test_clear() {
    let mut notepad = Notepad::new();
    notepad.add_text("Hello, world!");
    notepad.clear();
    assert_eq!(notepad.get_content(), "");
  }
}