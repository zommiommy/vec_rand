use core::{iter::Iterator, ops::Add, ops::AddAssign};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    #[inline]
    fn zero() -> f32 {
        0.0f32
    }
}

impl Zero for f64 {
    #[inline]
    fn zero() -> f64 {
        0.0f64
    }
}

impl Zero for u32 {
    #[inline]
    fn zero() -> u32 {
        0u32
    }
}

impl Zero for u64 {
    #[inline]
    fn zero() -> u64 {
        0u64
    }
}

impl Zero for i32 {
    #[inline]
    fn zero() -> i32 {
        0i32
    }
}

impl Zero for i64 {
    #[inline]
    fn zero() -> i64 {
        0i64
    }
}

impl Zero for usize {
    #[inline]
    fn zero() -> usize {
        0usize
    }
}

impl Zero for isize {
    #[inline]
    fn zero() -> isize {
        0isize
    }
}

impl Zero for u16 {
    #[inline]
    fn zero() -> u16 {
        0u16
    }
}

impl Zero for i16 {
    #[inline]
    fn zero() -> i16 {
        0i16
    }
}

impl Zero for u8 {
    #[inline]
    fn zero() -> u8 {
        0u8
    }
}

impl Zero for i8 {
    #[inline]
    fn zero() -> i8 {
        0i8
    }
}

#[inline]
/// Computes implace the cumulative sum of a vector of additive values.
/// 
/// # Example
/// 
/// ```
/// use vec_rand::cumsum_f32::cumsum_unrolled;
/// 
/// let mut weights = vec![1.0, 2.0, 3.0, 4.0];
/// cumsum_unrolled(&mut weights);
/// 
/// assert_eq!(weights, vec![1.0, 3.0, 6.0, 10.0]);
/// ```
pub fn cumsum_unrolled<F: Zero + AddAssign + Add<F, Output = F> + Copy>(random_vec: &mut [F]) {
    if random_vec.len() <= 0 {
        return;
    }

    let mut offset = F::zero();

    let max = if random_vec.len() >= 4 {
        if random_vec.len() % 4 == 0 {
            random_vec.len()
        } else {
            random_vec.len() - 4
        }
    } else {
        0
    };
    for i in (0..max).step_by(4) {
        let mut a = random_vec[i];
        let mut b = random_vec[i + 1];
        let mut c = random_vec[i + 2];
        let mut d = random_vec[i + 3];

        d += c + b + a + offset;
        c += b + a + offset;
        b += a + offset;
        a += offset;

        random_vec[i] = a;
        random_vec[i + 1] = b;
        random_vec[i + 2] = c;
        random_vec[i + 3] = d;

        offset = d;
    }

    let n = random_vec.len() - (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            random_vec[n] = random_vec[n] + offset;
        }
        2 => {
            random_vec[n] = random_vec[n] + offset;
            random_vec[n + 1] = random_vec[n + 1] + random_vec[n];
        }
        3 => {
            random_vec[n] = random_vec[n] + offset;
            random_vec[n + 1] = random_vec[n + 1] + random_vec[n];
            random_vec[n + 2] = random_vec[n + 2] + random_vec[n + 1];
        }
        _ => {}
    };
}
