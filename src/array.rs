/// Enumerate function for arrays. Not terribly efficient.
pub fn enumerate<T: Copy, const COUNT: usize>(arr: [T; COUNT]) -> [(usize, T); COUNT] {
    let mut out = [None; COUNT];

    for i in 0..COUNT {
        out[i] = Some((i, arr[i]));
    }

    out.map(|el| el.unwrap())
}

/// Zip function for arrays. Not terribly efficient.
pub fn zip<T: Copy, U: Copy, const COUNT: usize>(
    left: [T; COUNT],
    right: [U; COUNT],
) -> [(T, U); COUNT] {
    let mut arr = [None; COUNT];

    for i in 0..COUNT {
        arr[i] = Some((left[i], right[i]));
    }

    arr.map(|el| el.unwrap())
}
