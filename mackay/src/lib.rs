pub mod encodings {
pub trait Encoder<Symbol> {
    fn encode(&self, message: &[Symbol]) -> Vec<Symbol>;
    fn decode(&self, message: &[Symbol]) -> Vec<Symbol>;
}

pub struct RN {
    n: u8,
}

impl RN {
    pub fn make_rn(n: u8) -> Result<RN, ()> {
        if n % 2 == 0 {
            Err(())
        } else {
            Ok(RN {n: n})
        }
    }
}

impl Encoder<bool> for RN {
    fn encode(&self, message: &[bool]) -> Vec<bool> {
        let mut result = vec![];
        result.reserve(self.n as usize * message.len());
        for &c in message {
            match c {
                true => {
                    for _ in 0..self.n {
                        result.push(true)
                    }},
                false => {
                    for _ in 0..self.n {
                        result.push(false)
                    }
                }
            }
        }
        result
    }

    fn decode(&self, message: &[bool]) -> Vec<bool> {
        let mut result = vec![];
        let decoded_length = message.len() / (self.n as usize);
        result.reserve(decoded_length);
        if (message.len() % (self.n as usize)) != 0 || self.n == 0 {
            result
        } else {
            for i in 0..decoded_length {
                let mut count: usize = 0;
                for j in 0..self.n {
                    count += if message[i * self.n as usize + j as usize] == true {1} else {0};
                }
                result.push(count >= self.n as usize / 2);
            }
            result
        }
    }
}

}

#[cfg(test)]
mod tests {
    use encodings;
    use encodings::Encoder;
    #[test]
    fn rn() {
        let r3 = encodings::RN::make_rn(3).unwrap();
        
        let cleartext1 = vec![];
        let result1 = r3.encode(&cleartext1);
        assert_eq!(result1, []);
        assert_eq!(&r3.decode(&result1), &cleartext1);

        let cleartext2 = vec![true];
        let result2 = r3.encode(&cleartext2);
        assert_eq!(result2, [true, true, true]);
        assert_eq!(&r3.decode(&result2), &cleartext2);

        let cleartext3 = vec![true, false, true];
        let result3 = r3.encode(&cleartext3);
        assert_eq!(result3, [true, true, true, false, false, false, true, true, true]);
        assert_eq!(&r3.decode(&result3), &cleartext3);
    }
}
