use rand::{thread_rng, Rng};
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Sentence generator")
    .version("1.0")
    .author("jan@limpens.com")
    .about("Gives you a selection of words from a given word list.")
    .arg(Arg::with_name("INPUT")
         .help("Sets the word list to use")
         .required(true)
         .index(1))
    .arg(Arg::with_name("number")
         .short("n")
         .long("number")
         .default_value("3")
         .help("Sets how many words are returned"))
    .get_matches();

    let words = matches.value_of("INPUT").unwrap();
    let number_of_words = matches.value_of("number").unwrap();
    let mut number_of_words:u32 = number_of_words.parse().unwrap();

    let word_iter = words.split_whitespace();
    let mut vec: Vec<&str> = word_iter.collect();
    vec.sort();
    vec.dedup();
    let words_size: usize = vec.len();

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

