#![feature(box_syntax, env, fs, io, path)]

extern crate vword;
use vword::vword::VWord;

use std::fs::File;
use std::io::{BufReader, BufReadExt};
use std::path::Path;

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
    println!("word count {}", words.fold(0usize, |a,_| a + 1));
}

fn print_words(words: Box<Iterator<Item=VWord>>) {
    for word in words.map(|w| w.word) {
        println!("{}", word);
    }
}

fn get_words<'a>(req_ordered: bool) -> Box<Iterator<Item=VWord> + 'a> {
    box BufReader::new(File::open(&Path::new("/usr/share/dict/words")).unwrap())
        .lines()
        .map(|l| VWord::new(l.unwrap()))
        .filter(move |vword| if req_ordered {
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
