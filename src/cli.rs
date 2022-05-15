use crate::task::{Project, Task};
use crate::{fio, Serializer};
use std::error::Error;
use std::{env, io};
use terminal_size::terminal_size;

pub struct Cli {
    command: String,
    args: Vec<String>,
}

impl Cli {
    pub fn default() -> Self {
        let mut args = env::args();

        args.next(); // skip program name

        let command = args.next().unwrap_or_else(|| "help".to_string());

        Cli {
            command,
            args: args.collect::<Vec<String>>(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match &self.command.to_lowercase()[..] {
            "list" => Cli::list(&self.args[..])?,
            "add" => Cli::add(&self.args[..])?,
            "del" | "delete" => Cli::delete(&self.args[..])?,
            _ => Cli::help(),
        }

        Ok(())
    }

    fn list(args: &[String]) -> Result<(), io::Error> {
        let (width, _) = terminal_size().unwrap();

        match args {
            [name, ..] => println!(
                "{}",
                fio::find_project(name)?
                    .unwrap()
                    .to_table_string(width.0 as usize)
            ),
            [] => fio::get_all_projects()?
                .iter()
                .for_each(|project| println!("{}", project.to_table_string(width.0 as usize))),
        }

        Ok(())
    }

    // tm add [Project] ` [Task Name] ` [Description] ` [Priority]
    fn add(args: &[String]) -> Result<(), io::Error> {
        if args.is_empty() {
            Cli::help();
        }

        println!("{}", &args[0]);

        let task = args[0].split_once(" ` ").unwrap();
        let project_name = task.0;

        let mut project = fio::find_project(project_name)
            .unwrap_or_else(|_| Some(Project::new(project_name.to_string())))
            .unwrap();
        project.add_task(Task::deserialize(format!("false ` {}", task.1).as_str()).unwrap());

        fio::save_project(project)?;

        Ok(())
    }

    fn delete(args: &[String]) -> Result<(), io::Error> {
        if let [project_name, index] = args {
            let index: usize = index.parse().unwrap();
            let mut project = fio::find_project(project_name)?.unwrap();

            project.remove_task(index);

            fio::save_project(project)?;
        } else if let [project_name] = args {
            fio::remove_project_by_name(project_name)?
        } else {
            Cli::help()
        }

        Ok(())
    }

    fn help() {
        println!(
            "TaskMaster - cli task manager\n\n\
            USAGE:\n\
                \ttm list or tm list [Project name]\n\
                \ttm add [Project name] ` [Task name] ` [Description of a task] ` [Priority]\n\
                \ttm delete [Project name] or tm delete [Project name]\n\
            "
        )
    }
}
