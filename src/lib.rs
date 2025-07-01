use std::path::PathBuf;
mod models;
mod scanner;
use clap::Parser;
mod cleaner;
mod utils;
use crate::{cleaner::handle_projects, scanner::scan_files};

/// A simple project cleaner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(long, default_value_t = false, value_parser = clap::builder::BoolValueParser::new(), short)]
    cargo: bool,
    #[arg(long, default_value_t = false, value_parser = clap::builder::BoolValueParser::new(), short)]
    node: bool,
}

pub fn run_cli() {
    let args = Args::parse();

    if !args.node && !args.cargo {
        println!(
            "Utiliza las flags \"-n\" o \"-c\" para especificar si quieres proyectos de cargo o node."
        );
    }

    let projects = scan_files(&args.path, args.cargo, args.node);
    let deleted = handle_projects(&projects);
    println!("PROYECTOS LIMPIADOS: {}", deleted)
}
