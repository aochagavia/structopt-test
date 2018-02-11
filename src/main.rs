#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Options {
    #[structopt(name = "remove")]
    RemoveFile(Remove),
    #[structopt(name = "rename")]
    RenameFile(Rename)
}

#[derive(StructOpt, Debug)]
struct Remove {
    #[structopt(name = "FILE")]
    path: String
}

#[derive(StructOpt, Debug)]
struct Rename {
    #[structopt(name = "FROM")]
    from: String,
    #[structopt(name = "TO")]
    to: String
}

fn main() {
    let opt = Options::from_args();
    println!("Options: {:?}", opt);
}
