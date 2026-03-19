mod rendering {
  pub mod shaders {
    pub fn compile_shader(source: &str) {
      println!("Compiling shader: {}", source);
    }

    pub fn apply_shader(shader_id: u32) {
      println!("Applying shader {}", shader_id);
    }
  }

  pub mod graphics {
    pub fn draw_triangle() {
      println!("Drawing triangle");
    }

    pub fn draw_texture(texture: &str) {
      println!("Drawing texture: {}", texture);
    }

    pub fn clear_screen() {
      println!("Clearing screen");
    }
  }

  pub mod camera {
    pub struct Camera {
      position: (f32, f32, f32),
      target: (f32, f32, f32),
    }

    impl Camera {
      pub fn new() -> Self {
        Camera {
          position: (0.0, 0.0, 0.0),
          target: (0.0, 0.0, 0.0),
        }
      }

      pub fn look_at(&mut self, x: f32, y:f32, z:f32) {
        self.target = (x,y,z);
      }
    }
  }
}