mod physics {
  pub mod calculations {
    pub fn calculate_gravity(mass: f32) -> f32 {
      mass * 9.81
    }

    pub fn calculate_velocity(initial: f32, time: f32) -> f32 {
      initial + (9.81 * time)
    }

    pub fn validate_physics_params(mass: f32) -> bool {
      mass > 0.0
    }
  }

  pub mod collision {
    pub fn check_collision(obj1:&str, obj2: &str) -> bool {
      println!("Checking collision between {} and {}", obj1, obj2);
      false
    }

    pub fn resolve_collision(obj1: &str, obj2: &str) {
      println!("Resolving collision");
    }
  }
}