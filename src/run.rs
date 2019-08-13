
// OS & IO
use std::env;
use std::fs;
use std::io;

use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

use std::collections::HashMap;
use std::collections::LinkedList;

use std::str::FromStr;

// Usage
// retox file1.txt file2.txt -d ./somedir
// retox file1.txt file2.txt -s f=z
// replaces all f's with z's

pub fn run(config: Config) {
    println!("=== Starting Retox2 ===");

    /*
    // Create dummy data
    let config = dummy::new_config();
    let config = read_config(config);
    */

    // Run the core program
    let config = read_config(config);
    config.print_options();
    let config = validate_dirs(config).unwrap();
    let file_paths = read_dir_files(&config).unwrap();
    modify_files(file_paths, &config);
}

// Sets a default replacment sequence if non is specified
// TODO: Expand so it reads from a config file
fn read_config(mut config: Config) -> Config {
    if config.seqs.len() == 0 {
        config.seqs.push((" ", "_"));
    }

    config
}

pub struct Config<'a> {
    pub is_recursive: bool,
    pub is_force_invalid_dirs: bool,
    pub seqs: Vec<(&'a str, &'a str)>,
    pub dirs: Vec<PathBuf>,
}

impl<'a> Config<'a>{
    fn print_options(&self){
        debug!("=== Specified Options ===");
        debug!("is_recursive: {:?}", &self.is_recursive);
        debug!("is_force_invalid_dirs: {:?}", &self.is_force_invalid_dirs);
        debug!("replacment sequences: {:?}", &self.seqs);
        debug!("working directories: {:?}\n", &self.dirs);
        //debug!(": {:?}",);

    }
}

// Result<Vec<(PathBuf, String)>, &'static str>
fn modify_files(file_paths: LinkedList<PathBuf>, config: &Config) {
    //let changes = file_paths.into_iter();

    for mut path in file_paths {
        let temp_path = PathBuf::from(&path);
        path.set_file_name(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(config.seqs[0].0, config.seqs[0].1),
        );

        std::fs::rename(temp_path, path).unwrap();
    }

    /*
    // Gets file to change;
    println!("Files Without Spaces");
    let mut changes = changes.filter(|mut path: &PathBuf| path.file_name().unwrap()
                                                        .to_str().unwrap()
                                                        .contains(" "));
    println!("{:?}", changes);

    let mut changes = changes.for_each(|mut path: PathBuf| path.set_file_name(
        path.file_name().unwrap().to_str().unwrap().replace(" ", "_")
    ));


    println!("---------");
    println!("{:?}", changes);

    //let changes:Vec< = changes.into_iter().map(|file_path| path.to_str().replace(" ", "_");
    */
}

/// Receives a Vec<PathBuf> of possible directories and returns a Vec<PathBuf> of valid ones.
/// The function will panic on an invalid directory if the "force_invalid_dirs" = false.
/// Otherwise it will ignore the directory and continue building.
///
/// # Examples
/// ```
/// use retox2::{new_test_config, new_test_dirs, validate_dirs};
/// let config = dummy::new_config();
/// let dirs   = dummy::new_dirs(); // Creates 2 valid directories
/// let dirs   = validate_dirs(dirs, &config).unwrap();
/// assert!(dirs.len() == 2)
/// ```
pub fn validate_dirs(mut config: Config) -> Result<Config, &'static str> {
    let dirs = config.dirs;
    let mut valid_dirs: Vec<PathBuf> = vec![];

    for path in &dirs {
        if path.exists() && path.is_dir() {
            valid_dirs.push(path.clone());
        } else if !config.is_force_invalid_dirs {
            return Err("Invalid Directory Paths");
        }
    }

    // Sanity Check
    if dirs.len() <= 0 {
        return Err("The number of valid directories is 0. Please check your path names");
    }

    config.dirs = valid_dirs;
    Ok(config)
}

pub fn read_dir_files(config: &Config) -> Result<LinkedList<PathBuf>, &'static str> {
    info!("=== Displaying directory files ===");
    let dirs = config.dirs.clone();
    return read_dir_files_helper(dirs, LinkedList::new(), &config);
}

//#[dead_code]
fn read_dir_files_helper(
    dirs: Vec<PathBuf>,
    mut file_paths: LinkedList<PathBuf>,
    config: &Config,
) -> Result<LinkedList<PathBuf>, &'static str> {
    let mut inner_dirs = vec![];

    for entries in &dirs {
        let entries = entries.read_dir().unwrap();
        entries.for_each(|i| {
            if let Ok(entry) = i {
                if entry.path().is_dir() == true {
                    inner_dirs.push(entry.path())
                } else {
                    file_paths.push_back(entry.path())
                }
            }
        });
    }


    info!("Inner Dirs: {:?}", inner_dirs);
    info!("All  Files: {:?}\n", file_paths);

    // Recusively search directories
    if inner_dirs.len() != 0 {
        if config.is_recursive {
            return read_dir_files_helper(inner_dirs, file_paths, config);
        }
    }

    // Return function
    if file_paths.len() == 0 {
        return Err("No files to found");
    } else {
        return Ok(file_paths);
    }
}

pub mod dummy {
    use super::*;
    use std::path::PathBuf;

    pub fn new_config<'a>() -> Config<'a> {
        let config: Config = Config {
            is_recursive: true,
            is_force_invalid_dirs: false,
            seqs: vec![],
            dirs: new_dirs(),
        };

        config
    }

    pub fn new_dirs() -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = vec![];
        dirs.push(PathBuf::from("/home/creator-76/bc/retox2/test"));
        //dirs.push(PathBuf::from("/home/creator-76/bc/retox2/test2"));

        dirs
    }
}

#[cfg(test)]
pub mod tests {}
