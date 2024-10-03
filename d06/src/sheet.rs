pub(crate) struct Race {
    pub(crate) time: u64,
    pub(crate) distance: u64,
}

impl Race {
    pub(crate) fn ways_to_win(&self) -> usize {
        (1..self.time)
            .map(|t| t * (self.time - t))
            .filter(|dist| dist > &self.distance)
            .count()
    }
}
