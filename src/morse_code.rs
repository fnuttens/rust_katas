use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
            m
        }
     };
);

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: map! {
                ".-"    => "A", "-..."  => "B", "-.-."  => "C", "-.."   => "D",
                "."     => "E", "..-."  => "F", "--."   => "G", "...."  => "H",
                ".."    => "I", ".---"  => "J", "-.-"   => "K", "--"    => "M",
                "-."    => "N", "---"   => "O", ".--."  => "P", "--.-"  => "Q",
                ".-."   => "R", "..."   => "S", "-"     => "T", "..-"   => "U",
                "...-"  => "V", ".--"   => "W", "-..-"  => "X", "-.--"  => "Y",
                "--.."  => "Z",
                ".----" => "1", "..---" => "2", "...--" => "3", "....-" => "4",
                "....." => "5", "-...." => "6", "--..." => "7", "---.." => "8",
                "----." => "9", "-----" => "0"
            },
        }
    }

    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|w| self.decode_word(w))
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn decode_word(&self, morse_word: &str) -> String {
        morse_word
            .split_ascii_whitespace()
            .map(|morse_char| self.morse_code.get(morse_char).unwrap().clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }
}
