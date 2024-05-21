use indexmap::IndexSet;

#[derive(Debug, Clone)]
pub struct EnityBase {
  name: String,
  speed: f32,
  destroy: bool,
  /// default 1  
  /// no collision: 0
  collision: usize,
  groups: IndexSet<String>,
}

impl EnityBase {
  //normal
  pub fn name(&self) -> String {
    self.name.clone()
  }

  //destroy
  pub fn is_destroy(&self) -> bool {
    self.destroy
  }
  pub fn destroy(&mut self) {
    self.destroy = true;
  }

  //speed
  pub fn speed(&self) -> f32 {
    self.speed
  }
  pub fn set_speed(&mut self, speed: f32) {
    self.speed = speed
  }

  //collision
  pub fn get_collision(&self) -> usize {
    self.collision
  }
  pub fn set_collision(&mut self, level: usize) {
    self.collision = level
  }
  pub fn set_no_collision(&mut self) {
    self.collision = 0
  }

  //group
  pub fn groups(&self) -> Vec<String> {
    self.groups.iter().cloned().collect()
  }
  pub fn has_group(&self,group: &str) -> bool {
    self.groups.contains(group)
  }
  pub fn add_group(&mut self,group: &str) -> bool {
    self.groups.insert(group.to_string())
  }
  pub fn remove_group(&mut self,group: &str) -> bool {
    self.groups.swap_remove(group)
  }

  pub fn new(name: String, groups: Vec<String>, speed: f32) -> EnityBase {
    let groups = IndexSet::from_iter(groups);
    EnityBase {
      name,
      speed,
      groups,
      collision: 1,
      destroy: false,
    }
  }
}
