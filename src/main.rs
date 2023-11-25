use itertools::Itertools;
use markov::Chain;

fn main() {
    let corpus = include_str!("../corpi/fanged_noumena.txt");
    let filtered = corpus
        .lines()
        .skip(453)
        .filter(|line| !(line.starts_with(|c: char| c.is_ascii_digit()) && line.ends_with('.')))
        .map(|line| {
            line.strip_suffix(|c: char| c.is_ascii_digit())
                .unwrap_or(line)
        })
        .map(|line| line.strip_prefix("[[ ]] ").unwrap_or(line))
        .map(|line| {
            line.replace(". ", ".\n")
                .replace("! ", "!\n")
                .replace("? ", "?\n")
        })
        .flat_map(|s| s.split('\n').map(str::to_string).collect_vec())
        .map(|s| s.trim().to_string())
        .filter(|line| line.len() > 3)
        .fold(Chain::of_order(4), |mut chain, sentence| {
            chain.feed_str(&sentence);
            chain
        });

    // let mut chain = Chain::of_order(4);
    // chain.feed_str(&filtered);

    // // println!("{filtered}");

    for _ in 0..100 {
        println!("{:?}", filtered.generate_str());
    }
}
