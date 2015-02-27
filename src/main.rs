#![feature(env)]

extern crate vword;
use vword::vword::VWord;

use std::fs::File;
use std::io::BufReader;

/// Count command instructs the program to list
/// only the count of matching words. List command
/// instructs the program to actually go ahead and
/// list everything. The bool carried by each variant
/// determines whether valid words are defined by
/// the presence of vowels or ordered vowels.
enum Command {
    Count(bool),
    List(bool),
}

fn main() {
    let command = read_command(&std::env::args().collect::<Vec<_>>());

    match command {
        Command::Count(req_ordered) => print_count(get_words(req_ordered)),
        Command::List(req_ordered) => print_words(get_words(req_ordered)),
    };
}

fn print_count(words: Box<Iterator<Item=VWord>>) {

}

fn print_words(words: Box<Iterator<Item=VWord>>) {

}

fn get_words(req_ordered: bool) -> Box<Iterator<Item=VWord>> {
    BufReader::new(File::open(&Path::new("/usr/share/dict/words")))
        .lines()
        .map(VWord::new)
        .filter(|vword| if req_ordered {
            vword.has_ordered_vowels()
        } else {
            vword.has_all_vowels()
        })
}

fn read_command(args: &[String]) -> Command {
    let which = args.iter()
        .filter(|&i| &i[..] == &"count"[..] || &i[..] == &"list"[..])
        .nth(0)
        .unwrap_or(&"count".to_string())
        .clone();

    let ordered = args.iter().any(|i| i[..] == "ordered"[..] || i[..] == "sorted"[..]);

    match &&which[..] {
        &"count" => Command::Count(ordered),
        &"list" => Command::List(ordered),
        _ => Command::Count(false),
    }
}
