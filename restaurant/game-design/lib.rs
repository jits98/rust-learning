use self::physics::calculations;
use self::physics::collision;
use self::rendering::graphics;
use self::rendering::camera;
use self::rendering::shaders;
use self::input::keyboard;
use self::input::mouse;

pub mod physics {
  pub use super::calculations::calculate_gravity;

  pub use super::collision::check_collision;
}

pub mod render {
  pub use super::graphics::{draw_triangle, draw_texture, clear_screen};

  pub use super::camera::Camera;

  pub fn load_shader(source: &str) -> u32 {
    shaders::compile_shader(source);
    1
  }
}

pub mod input {
  pub use super::keyboard::is_key_pressed;
  pub use super::mouse::{get_mouse_position, is_button_pressed};
}