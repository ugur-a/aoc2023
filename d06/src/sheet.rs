pub(crate) struct Race {
    pub(crate) time: u64,
    pub(crate) distance: u64,
}

impl Race {
    pub(crate) fn ways_to_win(&self) -> usize {
        let mut options = (1..self.time).map(|t| t * (self.time - t));

        let min_time = options.position(|dist| dist > self.distance);

        if let Some(t_min) = min_time {
            // by the distance formula, `t_max := self.time - t_min`
            // and an off-by-one, because of course there is one
            let t_max = self.time as usize - t_min - 1;
            t_max - t_min
        } else {
            // no way to win the race
            0
        }
    }
}
