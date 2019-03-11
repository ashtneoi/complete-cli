use abbrev_tree::AbbrevTree;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::{
    BufRead,
    stdin,
    stdout,
};

#[derive(Serialize, Deserialize)]
struct CmdTree(AbbrevTree<Option<Box<CmdTree>>>);

fn main() {
    let d = ProjectDirs::from(
        "space.maia",
        "",
        "complete-cli",
    ).unwrap_or_else(|| panic!("can't get project dir"));

    let mut args = env::args();
    args.next().expect("missing arg 0? (shouldn't happen)");

    let mut p = d.config_dir().to_path_buf();

    let name = args.next().expect("missing program name");
    let write = if name == "-w" {
        p.push(&args.next().expect("missing program name"));
        if args.next().is_some() {
            panic!("too many args");
        }
        true
    } else {
        p.push(&name);
        false
    };

    if write {
        let f = File::create(&p).unwrap_or_else(
            |e| panic!("can't open '{}' ({})", p.to_str().unwrap(), e)
        );
    } else {
        let f = File::open(&p).unwrap_or_else(
            |e| panic!("can't open '{}' ({})", p.to_str().unwrap(), e)
        );
        let t: CmdTree = bincode::deserialize_from(stdin().lock()).unwrap();
    }
}
