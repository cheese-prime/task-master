use std::env;
use std::error::Error;

pub struct Cli {
    command: String,
    args: Vec<String>
}

impl Cli {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = env::args();

        args.next(); // skip program name

        let command = args.next()?;

        Ok(Cli { command, args: args.collect::<Vec<String>>() })
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match &self.command.to_lowercase()[..] {
            "list" => todo!(),
            "add" => todo!(),
            "del" | "delete" => todo!(),
            _ => todo!()
        }

        Ok(())
    }
}