pub trait VecExtension {
    fn count_increasing_pairs(&self) -> usize;
}

impl<T> VecExtension for Vec<T>
where
    T: Ord + Copy,
{
    fn count_increasing_pairs(&self) -> usize {
        self.windows(2)
            .filter(|window| {
                let lhs = window[0];
                let rhs = window[1];
                lhs < rhs
            })
            .count()
    }
}
