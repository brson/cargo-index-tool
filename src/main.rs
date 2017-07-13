#![allow(warnings)]

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate error_chain;

use structopt::StructOpt;
use errors::*;
use std::path::{PathBuf, Path};
use std::env;

mod home;

mod errors { error_chain! { } }

#[derive(StructOpt, Debug)]
#[structopt(name = "", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "r", long = "revdeps", help = "List crates by reverse deps")]
    revdeps: bool,

    #[structopt(short = "t", long = "trevdeps", help = "List crates by transitive reverse deps")]
    trevdeps: bool,

    #[structopt(short = "o", long = "one-point-oh", help = "List crates by 1.0 date")]
    one_point_oh: bool,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[structopt(help = "The local crates.io index, from ~/.cargo if omitted")]
    index: Option<String>,
}

quick_main!(run);

fn run() -> Result<()> {
    let opt = Opt::from_args();

    let index: Option<PathBuf> = opt.index.map(PathBuf::from);
    let index: Option<&Path> = index.as_ref().map(|p|&**p);
    
    if opt.revdeps {
        print_revdeps(index)?;
    } else if opt.trevdeps {
        print_trevdeps(index)?;
    } else if opt.one_point_oh {
        print_one_point_oh_crates(index)?;
    }
    
    Ok(())
}

// The name of the cargo index directory
const INDEX_DIR: &str = "github.com-1ecc6299db9ec823";

fn default_repo() -> Option<PathBuf> {
    home::cargo_home().map(|h| h.join("registry/index").join(INDEX_DIR))
}

fn print_revdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_trevdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_one_point_oh_crates(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn load_registry_json(index: Option<&Path>) -> Result<()> {
    let index = index.map(PathBuf::from).or(default_repo())
        .ok_or(Error::from("no crate index repo specified and unable to locate it ~/.cargo"))?;

    panic!()
}
