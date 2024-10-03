pub(crate) struct Race {
    pub(crate) time: u32,
    pub(crate) distance: u32,
}

impl Race {
    pub(crate) fn ways_to_win(&self) -> usize {
        (1..self.time)
            .map(|t| t * (self.time - t))
            .filter(|dist| dist > &self.distance)
            .count()
    }
}
