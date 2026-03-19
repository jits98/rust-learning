use game_engine::{physics, render, input};

fn main() {
  let gravity = physics::calculate_gravity(10.0);
  println!("Gravity force: {}", gravity);

  let mut camera = render::Camera::new();
  camera.look_at(0.0, 5.0, 10.0);

  render::clear_screen();
  render::draw_triangle();
  render::load_shader("vertex_shader.glsl");

  if input::is_key_pressed('W') {
    println!("Moving forward!");
  }

  let mouse_pos = input::get_mouse_position();
  println!("Mouse at: {:?}", mouse_pos);
}