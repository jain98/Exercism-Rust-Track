use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word = word.to_lowercase();
    let map = construct_map(&word);

    for w in possible_anagrams {
        let local_word = w.to_lowercase();
        //if word != local_word && word.len() == local_word.len() && is_anagram_sorting(&word, &local_word) {
        if word != local_word && word.len() == local_word.len() && is_anagram(&local_word, &map) {
            result.insert(*w);
        }
    }
    result
}

fn sort(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort();
    sorted_word
}

fn is_anagram(word: &str, map: &HashMap<char, i32>) -> bool {
    construct_map(word).eq(map)
}

fn is_anagram_sorting(word1: &str, word2: &str) -> bool {
    let mut tmp1 = word1.chars().collect::<Vec<char>>();
    tmp1.sort_unstable();
    let mut tmp2 = word2.chars().collect::<Vec<char>>();
    tmp2.sort_unstable();
    // println!("tmp1: {:?}", tmp1);
    // println!("tmp2: {:?}", tmp2);
    tmp1 == tmp2
}

fn construct_map(word: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    for c in word.chars() {
        let char_count = map.entry(c).or_insert(0);
        *char_count += 1;
    }

    map
}
