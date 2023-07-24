pub fn xorshift_u8(mut x: u8) -> u8 {
    const SHIFT_INDEX: usize = 0;

    x ^= x << SHIFTS[SHIFT_INDEX].0;
    x ^= x >> SHIFTS[SHIFT_INDEX].1;
    x ^= x << SHIFTS[SHIFT_INDEX].2;
    x
}

/// List of all the full-period shifts for 8-bit xorshift
const SHIFTS: [(u8, u8, u8); 24] = [
    (3, 1, 1),
    (2, 1, 1),
    (1, 1, 2),
    (1, 1, 3),
    (5, 1, 3),
    (6, 3, 5),
    (3, 1, 5),
    (7, 3, 5),
    (6, 7, 1),
    (4, 5, 3),
    (2, 5, 5),
    (5, 3, 6),
    (3, 5, 4),
    (5, 3, 7),
    (7, 5, 3),
    (3, 5, 5),
    (7, 7, 1),
    (5, 5, 2),
    (1, 7, 3),
    (3, 5, 7),
    (5, 5, 3),
    (3, 7, 1),
    (1, 7, 6),
    (1, 7, 7),
];
