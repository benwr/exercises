pub mod encodings {
use std::clone::Clone;

pub trait Encoder<Input: Clone, Output: Clone> {
    fn encode(&self, message: &[Input]) -> Vec<Output>;
    fn decode(&self, message: &[Output]) -> Vec<Input>;
}

pub struct Composition<Symbol> {
    components: Vec<Box<Encoder<Symbol, Symbol>>>,
}


impl<Symbol: Clone> Composition<Symbol> {
    pub fn make_composition() -> Composition<Symbol> {
        Composition {components: vec![]}
    }

    pub fn add_encoder(& mut self, new: Box<Encoder<Symbol, Symbol>>) {
        self.components.push(new);
    }
}

impl<Symbol: Clone> Encoder<Symbol, Symbol> for Composition<Symbol> {
    fn encode(&self, message: &[Symbol]) -> Vec<Symbol> {
        let mut result = vec![];
        result.extend_from_slice(message);
        let mut intermediate;
        for e in &self.components {
            intermediate = (*e).encode(&result);
            result = intermediate;
        }
        result
    }

    fn decode(&self, message: &[Symbol]) -> Vec<Symbol> {
        let mut result = vec![];
        result.extend_from_slice(message);
        let mut intermediate;
        for e in (&self.components).iter().rev() {
            intermediate = (*e).encode(&result);
            result = intermediate;
        }
        result
    }
}

#[derive(Debug, Clone)]
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

impl Encoder<bool, bool> for RN {
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

#[derive(Clone, Debug)]
pub struct Hamming74 {
}

impl Hamming74 {
    pub fn make_hamming74() -> Hamming74 {
        Hamming74 {}
    }
}

impl Encoder<bool, bool> for Hamming74 {
    /* This is not quite the Hamming(7,4) code as it was presented
       to me in MacKay. It has identical properties and is primarily
       a reordering that makes the code easier.
     */
    fn encode(&self, message: &[bool]) -> Vec<bool> {
        let mut result = vec![];
        for i in 0..message.len() / 4 {
            for j in 0..4 {
                if i * 4 + j < message.len() {
                    result.push(message[i * 4 + j]);
                } else {
                    result.push(false);
                }
            }
            for j in 0..3 {
                let mut acc = false;
                for k in 0..4 {
                    if k != j {
                        acc ^= message[i * 4 + k];
                    }
                }
                result.push(acc);
            }
        }
        result
    }

    fn decode(&self, message: &[bool]) -> Vec<bool> {
        let mut result = vec![];
        for i in 0..(message.len() / 7) {
            for j in 0..4 {
                result.push(message[i * 7 + j]);
            }
            let mut syndrome: u8 = 0;
            for j in 0..3 {
                let mut syndrome_i = message[i * 7 + 4 + j];
                for k in 0..4 {
                    if k != j {
                        syndrome_i ^= message[i * 7 + k];
                    }
                }
                syndrome |= (syndrome_i as u8) << j;
            }
            let flipped_bit : usize = match syndrome {
                0b011 => 0b10,
                0b101 => 0b01,
                0b110 => 0b00,
                0b111 => 0b11,
                _ => 0b11111111,
            };
            if flipped_bit != 0b11111111 {
                result[i * 7 + flipped_bit] = !result[i * 7 + flipped_bit];
            }
        }
        result
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
        let result4 = r3.decode(&cleartext3);
        assert_eq!(result4, [true]);
    }

    #[test]
    fn h74() {
        let h = encodings::Hamming74::make_hamming74();

        let cleartext1 = vec![];
        let result1 = h.encode(&cleartext1);
        assert_eq!(result1, []);
        assert_eq!(&h.decode(&result1), &cleartext1);

        let cleartext2 = vec![true, true, true, true];
        let result2 = h.encode(&cleartext2);
        assert_eq!(result2, [true, true, true, true, true, true, true]);
        assert_eq!(&h.decode(&result2), &cleartext2);

        let cleartext3 = vec![true, false, true, false, true, false, true,
         false];
        let result3 = h.encode(&cleartext3);
        assert_eq!(result3, [true, false, true, false, true, false, true, true,
         false, true, false, true, false, true]);
        assert_eq!(&h.decode(&result3), &cleartext3);

        let flipped = vec![true, true, true, true, true, false, false];
        assert_eq!(h.decode(&flipped), [false, true, true, true]);
        let flipped2 = vec![true, true, true, true, false, false, false];
        assert_eq!(h.decode(&flipped2), [true, true, true, false]);
    }

    #[test]
    fn composition() {
        let h = Box::new(encodings::Hamming74::make_hamming74());
        let r = Box::new(encodings::RN::make_rn(3).unwrap());
        let mut c = encodings::Composition::make_composition();
        c.add_encoder(r);
        c.add_encoder(h);

        let cleartext1 = vec![];
        let result1 = c.encode(&cleartext1);
        assert_eq!(result1, []);

        // TODO Actually test this composition
    }
}
