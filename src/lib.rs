pub mod vword {
    use std::char;

    #[derive(Debug)]
    pub struct VWord{
        word: String,
        vowels: [u8; 5],
    }

    impl VWord {
        pub fn new(word: String) -> VWord {
            let mut vword = VWord { word: word, vowels: [0; 5] };
            let mut count = 0;

            for c in vword.word.chars() {
                match c {
                    'a' => vword.vowels[count] = 'a' as u8,
                    'e' => vword.vowels[count] = 'e' as u8,
                    'i' => vword.vowels[count] = 'i' as u8,
                    'o' => vword.vowels[count] = 'o' as u8,
                    'u' => vword.vowels[count] = 'u' as u8,
                    _ => (),
                }
                count += 1;

                if count > 4 { break; }
            }

            vword
        }

        pub fn has_ordered_vowels(&self) -> bool {
            self.vowels == ['a' as u8, 'e' as u8, 'i' as u8, 'o' as u8, 'u' as u8]
        }

        pub fn has_all_vowels(&self) -> bool {
            let (a,e,i,o,u) = self.vowels.iter().fold((false, false, false, false, false), |(mut a, mut e, mut i, mut o, mut u), &c| {
                match char::from_u32(c.clone() as u32).unwrap() {
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
