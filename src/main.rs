#![allow(unused)]

use std::{
    collections::HashSet,
    fs::read_to_string,
    hash::Hash,
    io::{stdout, Write},
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn cook_word(word: &str) -> u32 {
    let mut word_rep = 0;
    for c in word.chars() {
        word_rep |= 1 << (c as usize - 'a' as usize);
    }
    word_rep
}

fn parse_word_list<const BUF_N: usize>(path: &Path) -> (Vec<String>, [u32; BUF_N]) {
    let words = read_to_string(path).unwrap();

    let mut res_mappings = Vec::new();
    let mut res_words = Vec::new();

    for word in words.split('\n') {
        let word = word.trim().to_ascii_lowercase();
        if word.len() == 5
            && word.chars().into_iter().any(|c| "aeiou".contains(c))
            && word.chars().into_iter().all(|c| c.is_alphabetic())
            && has_unique_elements(word.chars())
        {
            let word_rep = cook_word(&word);
            if res_mappings.contains(&word_rep) {
                continue;
            }
            res_words.push(word);
            res_mappings.push(word_rep);
        }
    }
    res_mappings.sort();

    let mut res = [0; BUF_N];
    for (i, map) in res_mappings.iter().enumerate() {
        res[i] = *map;
    }

    (res_words, res)
}

fn display_words(word_list: &Vec<String>, mapping: u32) {
    for i in 0..26 {
        if (1 << i) & mapping == 0 {
            print!("-");
        } else {
            print!("{}", char::from_u32('a' as u32 + i as u32).unwrap());
        }
    }
    let words: Vec<&str> = word_list
        .iter()
        .map(|w| &w[..])
        .filter(|w| cook_word(w) == mapping)
        .collect();
    println!(
        " {:?}",
        words
            .iter()
            .map(|w| w.to_string())
            .reduce(|a, w| a + " / " + &w)
            .unwrap()
    );
}

struct Solver<'a, const N: usize> {
    mappings: &'a Vec<u32>,
    indices: [usize; N],
    depth: usize,
}

impl<'a, const N: usize> Solver<'a, N> {
    fn new(mappings: &'a Vec<u32>) -> Self {
        Solver {
            mappings,
            indices: [0; N],
            depth: 0,
        }
    }
}

impl<'a, const N: usize> Iterator for Solver<'a, N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Find valid mapping for depth or decrease depth
            loop {
                // Check if index is out of range
                if self.indices[self.depth] >= self.mappings.len() {
                    // Decrease depth, increment upper and continue loop
                    // If depth == 0, then no more results can be found, return None
                    if self.depth == 0 {
                        return None;
                    }

                    self.depth -= 1;

                    self.indices[self.depth] += 1;
                    continue;
                }

                let mapping = self.mappings[self.indices[self.depth]];

                // Check if mapping is invalid
                if self
                    .indices
                    .iter()
                    .take(self.depth)
                    .any(|i| self.mappings[*i] & mapping != 0)
                {
                    // Invalid mapping: Increment index of current depth and continue
                    self.indices[self.depth] += 1;
                    continue;
                } else {
                    // Valid mapping: Break loop
                    break;
                }
            }

            // if self.depth == 0 {
            //     print!("\r\x1b[KProgress: {}", self.indices[0]);
            //     stdout().flush().unwrap();
            // }

            // Yield result or increase depth
            if self.depth == N - 1 {
                let res = self.indices;
                self.indices[N - 1] += 1;
                return Some(res);
            } else {
                self.depth += 1;
                self.indices[self.depth] = self.indices[self.depth - 1] + 1;
            }
        }
    }
}

fn main() {
    let word_path = PathBuf::from_str("wordle_words.txt").unwrap();
    let (word_list, mapping_list) = parse_word_list::<6000>(&word_path);

    println!("N words: {}\n", word_list.len());

    // let mut solver: Solver<5> = Solver::new(&mapping_list);
    // let t0 = Instant::now();

    // for r in solver {
    //     let dur = t0.elapsed();
    //     let sec = dur.as_secs();
    //     let min = sec / 60;
    //     let hr = min / 60;
    //     println!("\r\x1b[K[{:02}:{:02}:{:02}]", hr, min % 60, sec % 60);
    //     for i in r {
    //         let mapping = mapping_list[i];
    //         display_words(&word_list, mapping);
    //     }
    //     println!();
    // }

    solve_function(mapping_list, &word_list);
}

fn solve_function<const BUF_N: usize>(mapping_list: [u32; BUF_N], word_list: &Vec<String>) {
    let n = word_list.len();
    let t0 = Instant::now();
    let mut stdout = stdout();

    let mut n_res = 0;

    for i in 0..n {
        let a = mapping_list[i];

        print!("\x1b[A\r\x1b[K");
        display_words(&word_list, a);
        print!("\r\x1b[KProgress: {:5}", i);
        stdout.flush();

        for j in i + 1..n {
            let b = mapping_list[j];

            if a & b != 0 {
                continue;
            }
            let ab = a | b;

            for k in j + 1..n {
                let c = mapping_list[k];

                if ab & c != 0 {
                    continue;
                }
                let abc = ab | c;

                for l in k + 1..n {
                    let d = mapping_list[l];

                    if abc & d != 0 {
                        continue;
                    }
                    let abcd = abc | d;

                    for m in l + 1..n {
                        let e = mapping_list[m];

                        if abcd & e != 0 {
                            continue;
                        }

                        n_res += 1;
                        let dur = t0.elapsed();
                        let sec = dur.as_secs();
                        let min = sec / 60;
                        let hr = min / 60;
                        println!(
                            "\r\x1b[K\x1b[A\x1b[K[{:02}:{:02}:{:02}]",
                            hr,
                            min % 60,
                            sec % 60
                        );
                        display_words(&word_list, mapping_list[i]);
                        display_words(&word_list, mapping_list[j]);
                        display_words(&word_list, mapping_list[k]);
                        display_words(&word_list, mapping_list[l]);
                        display_words(&word_list, mapping_list[m]);
                        println!();
                        println!();
                    }
                }
            }
        }
    }
    println!("N results: {}", n_res);
}
