use clap::{command, Arg, ArgAction};
use std::error::Error;
use std::fmt::Display;
use std::fs::{read_dir, rename};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum RuntimeErrors {
    UnableToSetExtension,
}

impl Error for RuntimeErrors {}

impl Display for RuntimeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type ProcessFunc = dyn Fn(&PathBuf) -> Result<(), Box<dyn Error>>;

fn add_extension_to_file(path: &Path, extension: &str) -> Result<(), Box<dyn Error>> {
    let mut new_filename = path.clone().to_owned();
    let updated = new_filename.set_extension(extension);

    if !updated {
        return Err(Box::new(RuntimeErrors::UnableToSetExtension));
    }

    rename(path, &new_filename)?;
    println!("{:?} -> {:?}", path.file_name(), new_filename.file_name());
    Ok(())
}

fn add_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let kind = infer::get_from_path(&path)?;
    if let Some(kind) = kind {
        let extension = kind.extension();
        add_extension_to_file(&path, extension)?;
    } else {
        println!("skipping {:?}", path.as_os_str());
    }
    Ok(())
}

fn strip_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut new_filename = path.clone().to_owned();
    let updated = new_filename.set_extension("");

    if !updated {
        return Err(Box::new(RuntimeErrors::UnableToSetExtension));
    }

    rename(path, &new_filename)?;
    println!("{:?} -> {:?}", path.file_name(), new_filename.file_name());
    Ok(())
}

fn process_directory(path: &str, func: &ProcessFunc) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        func(&path)?;
    }

    Ok(())
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("strip")
                .short('s')
                .long("strip")
                .action(ArgAction::SetTrue)
                .help("Remove extensions rather than adding them"),
        )
        .arg(Arg::new("path").required(true))
        .get_matches();

    let folder_path = matches.get_one::<String>("path").expect("Path is not set");
    let strip = matches.get_flag("strip");
    let func = if strip {
        strip_extension
    } else {
        add_extension
    };
    process_directory(folder_path, &func).unwrap();
}
