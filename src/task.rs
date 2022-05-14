use crate::Serializer;
use colored::Colorize;
use std::cmp::Ordering;

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

    // name <= 40% of the output line
    // description >= 60% of the output line
    fn to_table_string(&self, id: usize, width: usize) -> String {
        let mut name = &self.name[..];
        let mut description = &self.description[..];
        let mut id = id;

        // for a visible space between name and description
        let width = width - 1;

        let name_max_len = (width as f64 * 0.4).floor() as usize - 1;
        let description_max_len = (width as f64 * 0.6).ceil() as usize;

        let mut result = String::new();

        let mut full_name = true;
        let mut full_desc = true;
        while full_name | full_desc {
            result.push_str(&format!(
                "{:<id_wid$} {:<name_max$} {:<desc_max$}\n",
                match id.cmp(&0) {
                    Ordering::Greater => {
                        let result = id.to_string();

                        id = 0;

                        result
                    }
                    _ => "".to_string(),
                },
                match name.len().cmp(&name_max_len) {
                    Ordering::Less => {
                        full_name = false;
                        let result = name;

                        name = &name[name.len()..];

                        result
                    }
                    _ => {
                        let result = &name[..name_max_len];
                        name = &name[name_max_len..];

                        result
                    }
                },
                match description.len().cmp(&description_max_len) {
                    Ordering::Less => {
                        full_desc = false;
                        let result = description;

                        description = &description[description.len()..];

                        result
                    }
                    _ => {
                        let result = &description[..description_max_len];

                        description = &description[description_max_len..];

                        result
                    }
                },
                name_max = name_max_len,
                desc_max = description_max_len,
                id_wid = 3
            ));
        }

        result
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

        let is_completed = matches!(temp.next().unwrap(), "true");
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

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn to_table_string(&self, width: usize) -> String {
        let id_width = 3;

        let mut result = format!(
            "{:>center$}{}\n",
            "",
            &self.name[..].black().on_white(),
            center = (width / 2 - self.name.len() / 2)
        );

        result.push_str(&format!(
            "{:<id_wid$} {:<name_wid$} {:<desc_wid$}\n",
            "ID".underline(),
            "Name".underline(),
            "Description".underline(),
            id_wid = id_width,
            name_wid = ((width - id_width) as f64 * 0.4) as usize - 1,
            desc_wid = ((width - id_width) as f64 * 0.6) as usize
        ));

        result.push_str(
            &self
                .tasks
                .iter()
                .enumerate()
                .map(|(index, task)| task.to_table_string(index + 1, width - id_width))
                .collect::<Vec<String>>()
                .join("\n"),
        );

        result
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
