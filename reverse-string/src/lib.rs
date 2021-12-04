use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    // Cannot reverse grapheme clusters
    //input.chars().rev().collect::<String>()
    let mut s = input.to_owned();
    reverse_grapheme_clusters_in_place(&mut s[..]);
    s
}
