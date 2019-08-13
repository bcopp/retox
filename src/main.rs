
use std::env;
use std::path::PathBuf;

// CMD Parser
#[macro_use]
extern crate clap;

// Logging
#[macro_use]
extern crate log;
extern crate stderrlog;

use clap::{App, Arg};

mod run;

fn main() {
    info!("Starting Program");
    let opts = App::new("Retox")
        .version("1.0")
        .author("Brendan C. <bcopp.oss@gmail.com>")
        .about("A program to rename large groups of files.")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .multiple(true)
                .takes_value(true)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::with_name("sequences")
                .short("s")
                .long("seq")
                .value_name("SEQ")
                .multiple(true)
                .takes_value(true)
                .value_delimiter(",")
                .help("Defines the characters to replace for each filename"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .help("Searches the specified directory recusively"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Does not print anything to stdout"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Specifies the verbosity of the program"),
        )
        .arg(Arg::with_name("force_invalid_dir").short("f").help(
            "Will cause the program to NOT throw an error if a specified directory is invalid",
        ))
        .get_matches();
    

    // Adjusts verbosity level to be useful
    let verbosity = match opts.occurrences_of("verbosity"){
        0 => 0,
        1 => 2,
        2 => 3,
        _ => 4,
    };

    // Set up logger
    stderrlog::new()
        .module(module_path!())
        .quiet(opts.is_present("quiet"))
        .verbosity(verbosity)
        .init()
        .unwrap();

    // Parse config optsions
    let is_recursive: bool = opts.is_present("recursive");
    let is_force_invalid_dir: bool = opts.is_present("force_invalid_dir");
    let dirs = match opts.values_of("directories") {
        Some(vals) => {
            let mut paths = vec![];
            for val in vals {
                paths.push(PathBuf::from(val));
            }
            paths
        }
        None => {
            let mut paths = vec![];
            paths.push(PathBuf::from("/home/creator-76/bc/retox2/test"));

            paths
        }
    };

    let seqs: Vec<(&str, &str)> = match opts.occurrences_of("sequences") {
        0 => vec![],
        1 => {
            let mut seqs: Vec<(&str, &str)> = vec![];
            let seq: Vec<&str> = opts.value_of("sequences").unwrap().split(":").collect();
            seqs.push((seq[0], seq[1]));

            seqs
        },
        _ => {
            let mut seqs: Vec<(&str, &str)> = vec![];
            let seq_stmts = opts.values_of("sequences").unwrap();
            for seq_stmt in seq_stmts {
                let seq: Vec<&str> = seq_stmt.split(":").collect();
                seqs.push((seq[0], seq[1]));
            }
            seqs
        }
    };

    let config = run::Config {
        is_recursive: is_recursive,
        is_force_invalid_dirs: is_force_invalid_dir,
        seqs: seqs,
        dirs: dirs,
    };

    run::run(config);
}
