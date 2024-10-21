use core::str::FromStr;
use std::collections::HashMap;

use crate::platform::Platform;

const N_CYCLES: usize = 1_000_000_000;

impl Platform {
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let mut p = Platform::from_str(file)?;

    let mut states = HashMap::new();

    let mut yet_to_run = None;
    for i in 0..N_CYCLES {
        p.cycle();

        if let Some(j) = states.insert(p.clone(), i) {
            eprintln!("cycle found: {j}->{i}");
            // let s(i) := state after round i
            // then:
            // we're in this condition
            // => s(i) = s(j)
            // => after round j, there are repeated cycles of length (i-j)
            // => for all k: s(j + (i-j)k) = s(i)
            // => s(n) = s((n-j)%(i-j))
            // => run for (N_STATES-j)%(i-j) _more_ cycles
            // => will achieve a state equivalent to s(N_STATES)
            yet_to_run = Some((N_CYCLES - 1 - j) % (i - j));
            break;
        }
    }

    if let Some(n) = yet_to_run {
        for _ in 0..n {
            p.cycle();
        }
    }

    Ok(p.north_load())
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 64)]
    #[test_case(REAL => 103_445)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
