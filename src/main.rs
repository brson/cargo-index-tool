#![allow(warnings)]

extern crate semver;
extern crate walkdir;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate error_chain;
extern crate serde_json;

use structopt::StructOpt;
use errors::*;
use std::path::{PathBuf, Path};
use std::env;
use walkdir::WalkDir;
use serde_json::Value as JsonValue;
use semver::{Version, VersionReq};
use std::collections::BTreeMap;

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
    #[structopt(long = "index", help = "The local crates.io index, from ~/.cargo if omitted")]
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

fn default_index() -> Option<PathBuf> {
    home::cargo_home().map(|h| h.join("registry/index").join(INDEX_DIR))
}

fn index_path(index: Option<&Path>) -> Result<PathBuf> {
    let index = index.map(PathBuf::from).or(default_index())
        .ok_or(Error::from("no crate index repo specified and unable to locate it ~/.cargo"))?;

    if !index.exists() {
        Err(format!("index {} does not exist", index.display()).into())
    } else {
        Ok(index)
    }
}

fn print_revdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_trevdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_one_point_oh_crates(index: Option<&Path>) -> Result<()> {
    let index = load_index(index)?;

    panic!()
}

fn load_index(index: Option<&Path>) -> Result<Index> {
    let index = index_path(index)?;

    for entry in WalkDir::new(index) {
        let entry = entry.chain_err(|| "unable to read dir entry")?;

        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().file_name() == Some("config.json".as_ref()) {
            continue;
        }

        
    }

    panic!()
}

type CrateName = String;
type FeatureName = String;
type Index = BTreeMap<CrateName, Vec<CrateVersion>>;

struct CrateVersion {
    name: String,
    vers: Version,
    deps: Vec<Dep>,
    cksum: String,
    features: BTreeMap<FeatureName, Vec<FeatureName>>,
    yanked: bool,
}

struct Dep {
    name: CrateName,
    req: VersionReq,
    features: Vec<FeatureName>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: String,
}

