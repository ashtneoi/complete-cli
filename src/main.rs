use abbrev_tree::AbbrevTree;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
    BufWriter,
    stdin,
    stdout,
};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
struct CmdTree(AbbrevTree<Option<Box<CmdTree>>>);

impl CmdTree {
    fn new() -> Self {
        Default::default()
    }
}

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
        let mut fw = BufWriter::new(&f);
        let mut t: CmdTree = CmdTree::new();
        for line in stdin().lock().lines() {
            let line = line
                .unwrap_or_else(|e| panic!("can't read line ({})", e));
            let mut wt = &mut t;
            let words = line.split(' ').filter(|x| x.len() > 0);
            for word in words {
                // TODO: When AbbrevTree learns not to add duplicate items,
                // this `if` can go away.
                if wt.0.get_mut(word).is_none() {
                    wt.0.add(word, Some(Box::new(CmdTree::new())));
                }
                wt = wt.0.get_mut(word).unwrap().as_mut().unwrap();
            }
        }
        // TODO: take() leaf data.
        bincode::serialize_into(&mut fw, &t)
            .unwrap_or_else(|e| panic!("can't write config ({})", e));
    } else {
        let f = File::open(&p).unwrap_or_else(
            |e| panic!("can't open '{}' ({})", p.to_str().unwrap(), e)
        );
        let mut fr = BufReader::new(&f);
        let t: CmdTree = bincode::deserialize_from(&mut fr).unwrap();
        let mut completed = Vec::new();
        let mut wt = &t;
        for word in args {
            let v = wt.0.complete(&word);
            if v.len() == 0 {
                panic!("no match for '{}'", word);
            } else if v.len() > 1 {
                panic!("multiple matches for '{}'", word);
            } else {
                completed.push(v[0].0.clone());
                wt = v[0].1.as_ref().unwrap();
            }
        }
        println!("{} {}", name, completed.join(" "));
    }
}
