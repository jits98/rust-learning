mod input {
  pub mod keyboard {
    pub fn is_key_pressed(key: char) -> bool {
      println!("Checking if {} is pressed", key);
      false
    }

    pub fn get_keyboard_state() -> String {
      "keyboard state".to_string()
    }
  }

  pub mod mouse {
    pub fn get_mouse_position() -> (i32, i32) {
      (100, 200)
    }

    pub fn is_button_pressed(button: u8) -> bool {
      println!("Checking mouse button {}", button);
      false
    }
  }
}