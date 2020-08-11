
use ::core::cmp::Ordering;
use super::xorshift;

// pub fn sample_biased(number: usize, seed: usize) -> usize {
//     
//     let scaling = u64::MAX() / number;
//     limit = range * scaling;
//     do
//        answer = random;
//     while ( answer >= limit);
//     return  answer / scaling;
// }