mod exit_codes;

use std::io;
use std::io::{Read};
use clap::Parser;
use kdl::{KdlDocument, KdlError, KdlNode};
use lazy_static::lazy_static;
use crate::exit_codes::exit;


#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, help = "Print version")]
    version: bool,
    #[arg(long, short, help = "Run a KDL query")]
    query: Option<String>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

enum RunMode {
    Version,
    Query,
    None,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
    let run_mode: RunMode = match &ARGS {
        _ if ARGS.version => RunMode::Version,
        _ if ARGS.query.is_some() => RunMode::Query,
        _ => RunMode::None,
    };
    match run_mode {
        RunMode::Version => println!("Shuddle version: {}", VERSION),
        RunMode::None => panic!("Nothing to do!"),
        RunMode::Query => { perform_query() }
    }
}

fn perform_query() {
    let mut buffer: String = String::new();
    if atty::isnt(atty::Stream::Stdin) {
        // We can just ignore the failure to capture piped content
        // the buffer will remain as "" in the event of failure
        let _ignored = io::stdin().read_to_string(&mut buffer);
    }
    let kdl_document: String = buffer;

    let kdl_parse: Result<KdlDocument, KdlError> = kdl_document.parse::<KdlDocument>();
    let kdl_parse: miette::Result<KdlDocument, miette::Report> = match kdl_parse {
        Ok(kdl) => Ok(kdl),
        Err(error) => Err(error.into()),
    };

    let kdl_document: KdlDocument = match kdl_parse {
        Ok(kdl) => {kdl}
        Err(error) => {
            eprintln!("{:?}", error);
            exit(2);
        }
    };

    // --query should never be None, as this function is conditional on it
    let query: &String = &ARGS.query.clone().expect("This shouldn't happen!");
    let results: Result<Option<&KdlNode>, KdlError> = kdl_document.query(query);

    let result: miette::Result<Option<&KdlNode>, miette::Report> = match results {
        Ok(results) => Ok(results),
        Err(error) => Err(error.into()),
    };
    let result: Option<&KdlNode> = match result {
        Ok(result) => {result}
        Err(error) => {
            eprintln!("{:?}", error);
            exit(2);
        }
    };
    let result: String = match result {
        Some(result) => result.to_string(),
        None => String::new(),
    };

    println!("{}", result);
}
