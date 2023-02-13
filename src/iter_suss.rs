use super::xorshift;

pub struct IterSuss<I> {
    seed: u64,
    father: I,
    pub bucket_size: usize,
    pub len: usize,
}

impl<T, I: Iterator<Item=T>> Iterator for IterSuss<I> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let bucket_size = self.bucket_size.min(self.len);

        let rng = self.seed as usize % bucket_size;
        // skip
        for _ in 0..rng.saturating_sub(1) {
            self.father.next();
        }
        let result = self.father.next();
        //skip
        for _ in (rng + 1)..bucket_size {
            self.father.next();
        }
        self.seed = xorshift(self.seed);
        self.len -= bucket_size;
        result
    }
} 

pub trait IteratorSuss<T>: Iterator<Item=T> + Sized {
    #[inline]
    fn suss(self, number_of_samples: usize, seed: u64, len: Option<usize>) -> IterSuss<Self> {
        let seed = xorshift(seed); 
        let (min, max) = self.size_hint();
        let len = len.or(max).unwrap_or(min);
        IterSuss{
            seed,
            father: self,
            bucket_size: (len as f32 / number_of_samples as f32).ceil() as usize,
            len,
        }
    }
}

impl<T, I: Iterator<Item=T>> IteratorSuss<T> for I {}