use crate::global::{NUM_COLOR, PALACE_SIZE};
use rand::Rng;
use tui::style::Color;

/// 90% => 2, 10% => 4
pub fn two_or_four() -> u128 {
    let mut rtr = rand::thread_rng();
    if rtr.gen_range(0..10) < 9 {
        2
    } else {
        4
    }
}

/// 生成坐标
pub fn position() -> (usize, usize) {
    let mut rtr = rand::thread_rng();
    let x = rtr.gen_range(0..PALACE_SIZE);
    let y = rtr.gen_range(0..PALACE_SIZE);
    (x, y)
}

/// 随机颜色
pub fn color() -> Color {
    let mut rtr = rand::thread_rng();
    let index = rtr.gen_range(0..NUM_COLOR.len());
    NUM_COLOR[index]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_or_four() {
        (1..=100).for_each(|_| println!("{:?}", two_or_four()));
    }
}
