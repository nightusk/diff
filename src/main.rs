use diff::Diff;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let diff = Diff::new(args[1].as_str(), args[2].as_str()).ond();
    println!("{} {}", args[1], args[2],);
    println!("ed:{}", diff.ed);
    println!("lcs:{}", diff.lcs);
    println!("ses:{:?}", diff.ses);
}
