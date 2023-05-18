const INPUT: &str = "cqjxjnds";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

// could have been a better DX if I used module instead of struct, but who cares?
struct PasswordManager {
    psw: String,
}

impl PasswordManager {
    pub fn new(input: &str) -> Self {
        Self {
            psw: input.to_string(),
        }
    }
    pub fn generate_next_valid(&mut self) -> String {
        self.psw = PasswordManager::increment_psw(&self.psw);
        while !PasswordManager::is_valid(&self.psw) {
            self.psw = PasswordManager::increment_psw(&self.psw)
        }
        self.psw.clone()
    }

    fn is_valid(psw: &str) -> bool {
        if psw.contains(['i', 'o', 'l']) {
            return false;
        }

        let (mut consecutive_letter, mut pairs_of_letters) = (0, 0);
        {
            let mut pause = 0;
            for (i, ch) in psw.chars().enumerate().skip(1) {
                let lch = psw.chars().nth(i - 1).unwrap();
                if ch == lch && pause == 0 {
                    pairs_of_letters += 1;
                    pause += 2;
                }

                if ALPHABET.chars().position(|c| c == ch).unwrap()
                    == ALPHABET.chars().position(|c| c == lch).unwrap() + 1
                {
                    consecutive_letter += 1
                } else if consecutive_letter < 2 {
                    consecutive_letter = 0
                }

                if pause > 0 {
                    pause -= 1
                }
            }
        }

        consecutive_letter >= 2 && pairs_of_letters >= 2
    }

    fn increment_psw(psw: &str) -> String {
        let decimal_incremented = psw.chars().rev().enumerate().fold(0, |acc, (unit, ch)| {
            let base26_value = ALPHABET.chars().position(|c| c == ch).unwrap();
            acc + base26_value * 26_usize.pow(unit as u32)
        }) + 1;

        let mut base26_incremented = vec![];
        let mut quotient = decimal_incremented;
        while quotient != 0 {
            base26_incremented.push(quotient % 26);
            quotient /= 26
        }

        let mut base26_str = String::new();
        for &d in base26_incremented.iter().rev() {
            base26_str.push(ALPHABET.chars().nth(d).unwrap())
        }
        base26_str
    }
}

fn main() {
    let mut password = PasswordManager::new(INPUT);

    let p1 = password.generate_next_valid();
    println!("{p1}");
    let p2 = password.generate_next_valid();
    println!("{p2}");
}
