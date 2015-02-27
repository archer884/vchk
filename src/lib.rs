pub mod vword {
    static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    #[derive(Debug)]
    pub struct VWord{
        pub word: String,
        vowels: [char; 5],
    }

    impl VWord {
        pub fn new(word: String) -> VWord {
            let mut vword = VWord { word: word, vowels: [0 as char; 5] };
            let mut count = 0;

            for c in vword.word.chars() {
                if VOWELS.iter().any(|&v| v == c) {
                    vword.vowels[count] = c;
                    count += 1;
                }
                if count > 4 { break; }
            }
            vword
        }

        pub fn has_ordered_vowels(&self) -> bool {
            self.vowels == VOWELS
        }

        pub fn has_all_vowels(&self) -> bool {
            let (a,e,i,o,u) = self.vowels.iter().fold((false, false, false, false, false), |(mut a, mut e, mut i, mut o, mut u), &c| {
                match c {
                    'a' => a = true,
                    'e' => e = true,
                    'i' => i = true,
                    'o' => o = true,
                    'u' => u = true,
                    _ => (),
                };
                (a,e,i,o,u)
            });
            a && e && i && o && u
        }
    }
}
