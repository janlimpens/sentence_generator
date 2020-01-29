use rand::{thread_rng, Rng};
extern crate clap;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("Sentence generator")
        .version("1.0")
        .author("jan@limpens.com")
        .about("Gives you a selection of words from a given word list.")
        .arg(
            Arg::with_name("words")
                .short("w")
                .help("Sets the word list from a string")
                .conflicts_with("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Sets the word list from a file")
                .required_unless("words")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .default_value("3")
                .help("Sets how many words are returned"),
        )
        .get_matches();

    let number_of_words = matches.value_of("number").unwrap();
    let number_of_words: usize = number_of_words.parse().unwrap();

    if matches.occurrences_of("words") > 0 {
        let words = matches.value_of("words").unwrap();
        process(words, number_of_words);
    } else if matches.occurrences_of("file") > 0 {
        let file_name = matches.value_of("file").unwrap();
        let words = fs::read_to_string(file_name).expect("Something went wrong reading the file");
        process(words.as_str(), number_of_words);
    } else {
        panic!("No input");
    }
}

fn process(words: &str, number_of_words: usize) {
    let mut number_of_words = number_of_words;
    let word_iter = words.split_whitespace();
    let mut vec: Vec<&str> = word_iter.collect();
    vec.sort();
    vec.dedup();
    let words_size: usize = vec.len();

    if words_size < number_of_words {
        panic!("You provided less words than required.");
    }

    let mut result = Vec::new();
    let mut rng = thread_rng();

    while number_of_words > 0 {
        let n: usize = rng.gen_range(0, words_size);
        if result.iter().any(|v| v == &n) == false {
            result.push(n);
            number_of_words = number_of_words - 1;
        }
    }
    for x in result {
        let word = vec[x];
        println!("{}", &word);
    }
}
