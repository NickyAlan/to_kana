use std::{collections::HashMap, vec};

pub fn romanji2syll(romaji: &str) -> Result<Vec<String>, ()> {
    //! convert romaji to syllables vector    
    // "minasan konnichiwa" -> ["mi", "na", "sa", "n", "ko", "n", "ni", "wa"]
    let text = romaji.to_lowercase().replace(" ", "");
    let mut syllables: Vec<String> = vec![];
    let mut current_word: Vec<char> = vec![];
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for c in text.chars() {
        current_word.push(c);
        if vowels.contains(&c) {
            //into String
            // "nko" -> "n", "ko"
            if current_word.len() > 2 {
                if current_word[0] == 'n' {
                    syllables.push('n'.to_string());
                    syllables.push(current_word[1..].iter().collect());
                // forgot "chi" or any not start with "n"
                } else {
                    syllables.push(current_word.iter().collect());
                }
            } else {
                syllables.push(current_word.iter().collect());
            }
            current_word.clear();
        }
    }

    if !current_word.is_empty() {
        syllables.push(current_word.iter().collect());
    }
    Ok(syllables)
}

pub fn extract_ambigous_kana(sysll: Vec<String>) -> Result<Vec<Vec<String>>, ()> {
    let ambigous_vec: Vec<String> = vec!["wa".to_string(), "su".to_string(), "zu".to_string(), "ja".to_string(), "ju".to_string(), "jo".to_string(), "ji".to_string(), "o".to_string()];
    let mut kanas_vec: Vec<Vec<String>> = vec![sysll.clone()];
    let mut v2_word: String;
    for (idx, c) in sysll.iter().enumerate() {
        if ambigous_vec.contains(&c) {
            // masu -> masu
            // su -> tsu
            if c == &"su".to_string() {
                // not start with su: will error at [idx-1]
                if idx > 0 {
                    if sysll[idx-1] == "ma" || sysll[idx-1] == "de" {
                        continue;
                    } else {
                        v2_word = "tsu".to_string();
                    }
                } else {
                    v2_word = "tsu".to_string();
                }
            } else if c == &"wa".to_string() {
                v2_word = "ha".to_string();    
            } else if c == &"o".to_string() {
                v2_word = "wo".to_string();
            } else {
                v2_word = format!("{}2", c);
            }

            let mut v2_vec = sysll.clone();
            v2_vec[idx] = v2_word;
            kanas_vec.push(v2_vec);
        }
    }
    Ok(kanas_vec)
}

pub fn syll2kana(syll: Vec<String>, mapper: &str) -> Result<Vec<&str>, ()> {
    // mapper: hiraga, katakana
    let hiragana = HashMap::from([
        ("a", "あ"),
        ("i", "い"),
        ("u", "う"),
        ("e", "え"),
        ("o", "お"),
        ("ka", "か"),
        ("ki", "き"),
        ("ku", "く"),
        ("ke", "け"),
        ("ko", "こ"),
        ("sa", "さ"),
        ("shi", "し"),
        ("su", "す"),
        ("se", "せ"),
        ("so", "そ"),
        ("ta", "た"),
        ("chi", "ち"),
        ("tsu", "つ"),
        ("te", "て"),
        ("to", "と"),
        ("na", "な"),
        ("ni", "に"),
        ("nu", "ぬ"),
        ("ne", "ね"),
        ("no", "の"),
        ("ha", "は"),
        ("hi", "ひ"),
        ("fu", "ふ"),
        ("he", "へ"),
        ("ho", "ほ"),
        ("ma", "ま"),
        ("mi", "み"),
        ("mu", "む"),
        ("me", "め"),
        ("mo", "も"),
        ("ya", "や"),
        ("yu", "ゆ"),
        ("yo", "よ"),
        ("ra", "ら"),
        ("ri", "り"),
        ("ru", "る"),
        ("re", "れ"),
        ("ro", "ろ"),
        ("wa", "わ"),
        ("wo", "を"),
        ("ga", "が"),
        ("gi", "ぎ"),
        ("gu", "ぐ"),
        ("ge", "げ"),
        ("go", "ご"),
        ("za", "ざ"),
        ("ji", "ぢ"),
        ("ji2", "じ"),
        ("zu", "ず"),
        ("ze", "ぜ"),
        ("zo", "ぞ"),
        ("da", "だ"),
        ("zu2", "づ"),
        ("de", "で"),
        ("do", "ど"),
        ("ba", "ば"),
        ("bi", "び"),
        ("bu", "ぶ"),
        ("be", "べ"),
        ("bo", "ぼ"),
        ("pa", "ぱ"),
        ("pi", "ぴ"),
        ("pu", "ぷ"),
        ("pe", "ぺ"),
        ("po", "ぽ"),
        ("n", "ん"),
        ("kya", "きゃ"),
        ("kyu", "きゅ"),
        ("kyo", "きょ"),
        ("sha", "しゃ"),
        ("shu", "しゅ"),
        ("sho", "しょ"),
        ("cha", "ちゃ"),
        ("chu", "ちゅ"),
        ("cho", "ちょ"),
        ("nya", "にゃ"),
        ("nyu", "にゅ"),
        ("nyo", "にょ"),
        ("hya", "ひゃ"),
        ("hyu", "ひゅ"),
        ("hyo", "ひょ"),
        ("mya", "みゃ"),
        ("myu", "みゅ"),
        ("myo", "みょ"),
        ("rya", "りゃ"),
        ("ryu", "りゅ"),
        ("ryo", "りょ"),
        ("gya", "ぎゃ"),
        ("gyu", "ぎゅ"),
        ("gyo", "ぎょ"),
        ("ja", "じゃ"),
        ("ju", "じゅ"),
        ("jo", "じょ"),
        ("ja2", "ぢゃ"),
        ("ju2", "ぢゅ"),
        ("jo2", "ぢょ"),
        ("bya", "びゃ"),
        ("byu", "びゅ"),
        ("byo", "びょ"),
        ("pya", "ぴゃ"),
        ("pyu", "ぴゅ"),
        ("pyo", "ぴょ"),
    ]);

    let katakana = HashMap::from([
        ("a", "ア"),
        ("i", "イ"),
        ("u", "ウ"),
        ("e", "エ"),
        ("o", "オ"),
        ("ka", "カ"),
        ("ki", "キ"),
        ("ku", "ク"),
        ("ke", "ケ"),
        ("ko", "コ"),
        ("sa", "サ"),
        ("shi", "シ"),
        ("su", "ス"),
        ("se", "セ"),
        ("so", "ソ"),
        ("ta", "タ"),
        ("chi", "チ"),
        ("tsu", "ツ"),
        ("te", "テ"),
        ("to", "ト"),
        ("na", "ナ"),
        ("ni", "ニ"),
        ("nu", "ヌ"),
        ("ne", "ネ"),
        ("no", "ノ"),
        ("ha", "ハ"),
        ("hi", "ヒ"),
        ("fu", "フ"),
        ("he", "ヘ"),
        ("ho", "ホ"),
        ("ma", "マ"),
        ("mi", "ミ"),
        ("mu", "ム"),
        ("me", "メ"),
        ("mo", "モ"),
        ("ya", "ヤ"),
        ("yu", "ユ"),
        ("yo", "ヨ"),
        ("ra", "ラ"),
        ("ri", "リ"),
        ("ru", "ル"),
        ("re", "レ"),
        ("ro", "ロ"),
        ("wa", "ワ"),
        ("wo", "ヲ"),
        ("kya", "キャ"),
        ("kyu", "キュ"),
        ("kyo", "キョ"),
        ("sha", "シャ"),
        ("shu", "シュ"),
        ("sho", "ショ"),
        ("cha", "チャ"),
        ("chu", "チュ"),
        ("cho", "チョ"),
        ("nya", "ニャ"),
        ("nyu", "ニュ"),
        ("nyo", "ニョ"),
        ("hya", "ヒャ"),
        ("hyu", "ヒュ"),
        ("hyo", "ヒョ"),
        ("mya", "ミャ"),
        ("myu", "ミュ"),
        ("myo", "ミョ"),
        ("rya", "リャ"),
        ("ryu", "リュ"),
        ("ryo", "リョ"),
        ("ga", "ガ"),
        ("gi", "ギ"),
        ("gu", "グ"),
        ("ge", "ゲ"),
        ("go", "ゴ"),
        ("za", "ザ"),
        ("ji", "ジ"),
        ("zu", "ズ"),
        ("ze", "ゼ"),
        ("zo", "ゾ"),
        ("da", "ダ"),
        ("ji2", "ヂ"),
        ("zu2", "ヅ"),
        ("de", "デ"),
        ("do", "ド"),
        ("ba", "バ"),
        ("bi", "ビ"),
        ("bu", "ブ"),
        ("be", "ベ"),
        ("bo", "ボ"),
        ("pa", "パ"),
        ("pi", "ピ"),
        ("pu", "プ"),
        ("pe", "ペ"),
        ("po", "ポ"),
        ("n", "ン"),
        ("gya", "ギャ"),
        ("gyu", "ギュ"),
        ("gyo", "ギョ"),
        ("ja", "ジャ"),
        ("ju", "ジュ"),
        ("jo", "ジョ"),
        ("ja2", "ヂャ"),
        ("ju2", "ヂュ"),
        ("jo2", "ヂョ"),
        ("bya", "ビャ"),
        ("byu", "ビュ"),
        ("byo", "ビョ"),
        ("pya", "ピャ"),
        ("pyu", "ピュ"),
        ("pyo", "ピョ"),
    ]);

    
    let mut kana_vec: Vec<&str> = vec![];
    if mapper == "hiragana" {   
        for word in &syll {
            if let Some(kana) = hiragana.get(word.trim()) {
                kana_vec.push(kana);
            } 
        }
    } else if mapper == "katakana" {
        for word in &syll {
            if let Some(kana) = katakana.get(word.trim()) {
                kana_vec.push(kana);
            } 
        }
    }
    Ok(kana_vec)
}

