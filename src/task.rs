#[derive(Debug)]
pub enum Priority {
    High,
    Medium,
    Low
}

#[derive(Debug)]
pub struct Task {
    name: String,
    description: String,
    priority: Priority,
    is_completed: bool
}

impl Task {
    pub fn new(name: String, description: String, priority: Priority) -> Self {
        Self { name, description, priority, is_completed: false }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }
}

#[derive(Debug)]
pub struct Project {
    name: String,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self { name, tasks: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
