use crate::{
    models::{ProjectEntry, Projects},
    utils::ask_yes_or_no,
};
use std::{fs, path::PathBuf};
pub fn handle_projects(projects: &Projects) -> u32 {
    let mut deleted: u32 = 0;
    for project in projects.entries.iter() {
        if delete_and_ask(project) {
            deleted += 1;
        }
    }
    deleted
}

pub fn delete_and_ask(project: &ProjectEntry) -> bool {
    if ask_yes_or_no(
        format!(
            "Te gustaria eliminar las dependencias de {} ({})",
            project.path.display(),
            project.project_type,
        )
        .as_str(),
    ) {
        match project.project_type {
            crate::models::ProjectType::Cargo => delete_with_handling(&project.path.join("target")),
            crate::models::ProjectType::Node => {
                delete_with_handling(&project.path.join("node_modules"))
            }
        }
        true
    } else {
        false
    }
}

pub fn delete_with_handling(path: &PathBuf) {
    match fs::remove_dir_all(path) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => println!("Permiso denegado"),
            std::io::ErrorKind::NotFound => {
                println!("Parece que node_modules o target no existen.")
            }
            other => println!("Error inesperado {:?}", other),
        },
    }
}
