use rand::{thread_rng, Rng};

fn main() {
    // get them by args    
    let mut number_of_words: u32 = 3;
    let mut words = String::new();
    words = "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string();
    
    let word_iter = words.split_whitespace();
    let vec: Vec<&str> = word_iter.collect();
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

