use std::collections::HashMap;
use std::thread;
use std::sync::Arc;

pub fn frequency(input: &[&str], n: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let mut threads = vec![];
    let data = input.iter().map(|s| s.to_ascii_lowercase()).collect::<Vec<String>>();
    let shared_input = Arc::new(data);

    for i in 0..n {
        let shared_input = Arc::clone(&shared_input);
        let chunk_size = (shared_input.len() + n - 1) / n;
        let start = if i*chunk_size > input.len() { break } else { i*chunk_size };
        let till_end = start + chunk_size > input.len();

        threads.push(thread::spawn(move || {
            let mut result: HashMap<char, usize> = HashMap::new();
            let slice = if till_end {
                &shared_input.as_slice()[start..]
            } else {
                &shared_input.as_slice()[start..start+chunk_size]
            };

            slice
                .into_iter()
                .for_each(|s| {
                    s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                        let v = result.entry(c).or_default();
                        *v += 1;
                    })
                });

            result
        }));
    }

    threads.into_iter().for_each(|t| {
        let m = t.join().unwrap();
        m.iter().for_each(|(k, v)| {
            let val = result.entry(*k).or_default();
            *val += v;
        });
    });

    result
}
