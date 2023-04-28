fn main() {
    let english = include_str!("../english.txt");
    let bahasa = include_str!("../bahasa.txt");
    for (i, (english, bahasa)) in english.lines().zip(bahasa.lines()).enumerate() {
        println!("{i}\t{bahasa}\t{english}");
    }
}

fn bahasa() {
    static BAHASA: &'static [u8] = include_bytes!("../small_id.msgpack");
    const NUMBER_OF_WORDS: usize = 10000;

    let mut bahasa_words: Vec<String> = Vec::with_capacity(NUMBER_OF_WORDS);
    let mut parsed = 0;
    for word in BAHASA
        .split(|byte| match byte {
            0xa0..=0xbf => true,
            _ => false,
        })
        .skip(4)
    {
        let word: String = String::from_utf8_lossy(word)
            .replace('�', "")
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .collect();
        if word.is_empty() {
            continue;
        }
        bahasa_words.push(word);
        parsed += 1;
        if parsed > NUMBER_OF_WORDS {
            break;
        }
    }
    std::fs::write(
        "bahasa.txt",
        bahasa_words
            .into_iter()
            .fold(String::with_capacity(NUMBER_OF_WORDS), |mut acc, word| {
                acc = format!("{acc}\n{word}");
                acc
            }),
    );
}
