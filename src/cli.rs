use crate::fio;
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

        let command = args.next().unwrap();

        Cli {
            command,
            args: args.collect::<Vec<String>>(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match &self.command.to_lowercase()[..] {
            "list" => Cli::list(&self.args[..])?,
            "add" => todo!(),
            "del" | "delete" => todo!(),
            _ => todo!(),
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
}
