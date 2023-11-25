use std::{fs::File, io::Write};

use itertools::Itertools;

fn main() {
    let corpus = include_str!("../corpi/fanged_noumena.txt");
    let filtered = corpus
        .lines()
        .skip(453)
        .map(str::trim)
        .filter(|line| line.len() > 3)
        .filter(|line| !(line.starts_with(|c: char| c.is_ascii_digit()) && line.ends_with('.')))
        .map(|line| {
            line.strip_suffix(|c: char| c.is_ascii_digit())
                .unwrap_or(line)
        })
        .map(|line| line.strip_prefix("[[ ]] ").unwrap_or(line))
        .take_while(|line| !line.starts_with("URBANOMIC"))
        .filter(|line| line.chars().filter(|c| c.is_ascii_lowercase()).count() > line.len() / 2)
        .filter(|line| !(line.starts_with('‘') && line.ends_with('.')))
        .join("\n");

    let mut output = File::create("filtered_noumena.txt").unwrap();
    output.write_all(filtered.as_bytes()).unwrap();
}
