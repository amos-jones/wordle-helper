mod cli;
mod wordle;

fn main() {
    let cli = cli::parse();
    let mut finder = wordle::WordFinder::new();

    if let Some(green) = cli.green {
        green.chars().enumerate().for_each(|(i, c)| {
            if c >= 'a' && c <= 'z' {
                finder.exact(c, i);
            }
        });
    }

    if let Some(red) = cli.red {
        red.chars().for_each(|c| {
            if c >= 'a' && c <= 'z' {
                finder.not_present(c);
            }
        })
    }

    if let Some(yellows) = cli.yellows {
        yellows.iter().for_each(|yellow| {
            yellow.chars().enumerate().for_each(|(i, c)| {
                if c >= 'a' && c <= 'z' {
                    finder.present(c, i);
                }
            })
        })
    }

    let possibilities = {
        let mut possibilities = finder
            .possibilities()
            .iter()
            .map(|w| wordle::WeightedString::new(w))
            .collect::<Vec<_>>();
        possibilities.sort();
        possibilities
    };

    possibilities
        .iter()
        .for_each(|poss| println!("{} - {}", poss.word, poss.score));
}
