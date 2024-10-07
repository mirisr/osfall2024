
fn get_word_index(word: &str, unique_words: &Vec<String>) -> i32 {
    for i in 0..unique_words.len() {
        if word == unique_words[i] {
            return i as i32;
        }
    }
    -1
}

fn find_max_word_and_frequency(unique_words: &Vec<String>, counts: &Vec<usize>) -> (String, usize) {
    let mut max_count = 0;
    let mut max_word = String::new();
    
    for i in 0..unique_words.len() {
        if counts[i] > max_count {
            max_count = counts[i];
            max_word = unique_words[i].clone(); 
        }
    }

    (max_word, max_count)
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    let mut unique_words: Vec<String> = Vec::new();  
    let mut counts: Vec<usize> = Vec::new();

    for word in words {
        let word_index = get_word_index(word, &unique_words);
        if word_index != -1 {
            counts[word_index as usize] += 1;
        } else {
            unique_words.push(word.to_string());
            counts.push(1);
        }
    }

    find_max_word_and_frequency(&unique_words, &counts)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}