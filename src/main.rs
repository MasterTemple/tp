use std::{collections::BTreeMap, fs, io, path::{Path, PathBuf}};

use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new project from a template
    New {
        /// The project name
        #[arg(short, long)]
        project: Option<String>,

        /// The template to use
        #[arg(short, long)]
        template: Option<String>,

        /// Whether or not to rename `main.typ` to `<Project>.typ`
        #[arg(short, long)]
        no_main: bool,
    },
    /// Initialize a new project in the current directory
    Init {
        /// The template to use
        #[arg(short, long)]
        template: Option<String>,

        /// Whether or not to rename `main.typ` to `<Project>.typ`
        #[arg(short, long)]
        no_main: bool,
    },
}

struct ParsedCmd {
    project: Option<String>,
    template: Option<String>,
    no_main: bool,
    use_current_directory: bool,
}

impl ParsedCmd {
    pub fn new(cmd: Commands) -> Self {
        match cmd {
            Commands::New { project, template, no_main } => {
                Self { project, template, no_main, use_current_directory: false }
            },
            Commands::Init { template, no_main } => {
                let current_dir = std::env::current_dir()
                    .expect("Failed to get current directory");
                let project = current_dir.file_name()
                    .expect("Invalid directory name")
                    .to_string_lossy()
                    .to_string();
                Self {
                    project: Some(project),
                    template,
                    no_main,
                    use_current_directory: true,
                }
            },
        }
    }
}

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

/// https://stackoverflow.com/a/65192210
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn tp_dir() -> AnyResult<PathBuf> {
    let mut dir = dirs_next::data_dir().ok_or_else(|| "Could not determine data directory.")?;
    dir.push("tp");
    Ok(dir)
}

fn template_dir() -> AnyResult<PathBuf> {
    Ok(tp_dir()?.join("templates"))
}


const INDEX_FILE: &'static str = "main.typ";

fn ensure_templates_exist() -> AnyResult<()> {
    let dir = template_dir()?;
    let none = dir.join("None");
    let none_index_file = none.join(INDEX_FILE);
    if !none.exists() {
        fs::create_dir_all(none)?;
    }
    fs::write(none_index_file, "")?;
    Ok(())
}

fn main() -> AnyResult<()> {

    ensure_templates_exist()?;
    let re = Regex::new("[^/]+$").unwrap();
    let args = Args::parse();

    let ParsedCmd { project, template, no_main, use_current_directory } = ParsedCmd::new(args.command);

    let dir = fs::read_dir(template_dir()?)?;
    let templates: BTreeMap<String, PathBuf> = dir.into_iter().filter_map(|file| {
        let file = file.ok()?;
        let file_type = file.file_type().ok()?;
        if !file_type.is_dir() { None? };

        let file_name = file.file_name();
        let template = re.find(file_name.to_str()?)?.as_str().to_string();
        let path = file.path();

        Some((template, path))
    }).collect();

    // prompt for project name
    let project = match project {
        Some(p) => p,
        None => Text::new("Enter a project name:").prompt()?
    };

    // check if template is valid
    let given_template = template.map(|template| {
        match templates.contains_key(&template) {
            true => Ok(template),
            false => {
                Err("Invalid template provided")
            },
        }
    }).transpose()?;

    // prompt for template
    let template = match given_template {
        Some(given) => given,
        None => Select::new("Select a template:", templates.keys().collect())
            .prompt()?.clone(),
    };


    let new_dir = if use_current_directory {
        PathBuf::from(".")
    } else {
        PathBuf::from(&project)
    };
    fs::create_dir_all(&new_dir)?;

    let template_dir = templates.get(&template).ok_or_else(|| "Invalid template selected.")?;
    copy_dir_all(template_dir, &new_dir)?;

    if no_main {
        fs::rename(new_dir.join(INDEX_FILE), new_dir.join(format!("{}.typ", &project)))?;
    }

    Ok(())
}
