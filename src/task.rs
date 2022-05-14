use crate::{Serializer, ToTableLikeString};
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Serializer for Priority {
    type Type = Self;

    fn serialize(self) -> String {
        match self {
            Priority::High => "High".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::Low => "Low".to_string(),
        }
    }

    fn deserialize(src: &str) -> Option<Self::Type> {
        match src {
            "High" => Some(Priority::High),
            "Medium" => Some(Priority::Medium),
            "Low" => Some(Priority::Low),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Task {
    name: String,
    description: String,
    priority: Priority,
    is_completed: bool,
}

impl Task {
    pub fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            is_completed: false,
        }
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

impl Serializer for Task {
    type Type = Task;

    fn serialize(self) -> String {
        format!(
            "{} ` {} ` {} ` {}",
            self.is_completed,
            self.name,
            self.description,
            self.priority.serialize()
        )
    }

    fn deserialize(src: &str) -> Option<Self::Type> {
        let mut temp = src.split(" ` ");

        if src.len() < 4 {
            return None;
        }

        let is_completed = match temp.next().unwrap() {
            "true" => true,
            _ => false,
        };
        let name = temp.next().unwrap().to_string();
        let description = temp.next().unwrap().to_string();
        let priority = match Priority::deserialize(temp.next().unwrap()) {
            None => return None,
            Some(value) => value,
        };

        Some(Task {
            is_completed,
            name,
            description,
            priority,
        })
    }
}

impl ToTableLikeString for Task {
    // name <= 40% of the output line
    // description >= 60% of the output line
    fn to_table_string(&self, width: usize) -> String {
        let name = &self.name[..];
        let description = &self.description[..];

        // for a visible space between name and description
        let width = width - 1;

        let name_max_len = (width as f64 * 0.4).floor() as usize;
        let description_max_len = (width as f64 * 0.6).ceil() as usize;

        format!(
            "{:>name_max$}{:>desc_max$}",
            name,
            description,
            name_max = name_max_len,
            desc_max = description_max_len
        )
    }
}

#[derive(Debug)]
pub struct Project {
    name: String,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

impl Serializer for Project {
    type Type = Project;

    fn serialize(self) -> String {
        let joined = {
            let mut buf = String::new();

            for task in self.tasks {
                buf += &task.serialize();
            }

            buf
        };

        format!("{}\n{}", self.name, joined)
    }

    fn deserialize(src: &str) -> Option<Self::Type> {
        let mut lines = src.split("\n");

        let name = match lines.next() {
            Some(val) => val,
            None => return None,
        }
        .to_string();

        Some(Project {
            name,
            tasks: lines.map(|line| Task::deserialize(line).unwrap()).collect(),
        })
    }
}

impl ToTableLikeString for Project {
    fn to_table_string(&self, width: usize) -> String {
        let id_width = (width as f64 * 0.1).ceil() as usize;

        self.tasks
            .iter()
            .enumerate()
            .map(|(index, task)| {
                format!(
                    "{:<wid$}{}",
                    index,
                    task.to_table_string(width - id_width),
                    wid = id_width
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
