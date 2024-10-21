pub mod p1;
pub mod p2;
pub mod step;

fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |hash, c| ((hash + c) * 17) % 256)
}
