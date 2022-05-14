// FILE INPUT/OUTPUT

use crate::task::Project;
use crate::{Serializer};
use std::path::Path;
use std::{fs, io};

const APP_DATA: &str = ".appdata";

pub fn save_project(project: Project) -> Result<(), io::Error> {
    fs::write(
        Path::new(&format!("{}/{}", APP_DATA, project.name())),
        project.serialize(),
    )?;

    Ok(())
}

pub fn find_project(name: &str) -> Result<Option<Project>, io::Error> {
    let contents = fs::read_to_string(format!("{}/{}", APP_DATA, name))?;

    Ok(Project::deserialize(&contents))
}

pub fn get_all_projects() -> Result<Vec<Project>, io::Error> {
    let prj_files = fs::read_dir(APP_DATA)?;

    Ok(prj_files
        .map(|file| {
            Project::deserialize(
                &fs::read_to_string(&format!(
                    "{}/{}",
                    APP_DATA,
                    file.unwrap().file_name().to_str().unwrap()
                ))
                .unwrap()[..],
            )
        })
        .map(|proj| proj.unwrap())
        .collect::<Vec<Project>>())
}
