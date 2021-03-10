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
    let palace_size = unsafe { PALACE_SIZE };
    let mut rtr = rand::thread_rng();
    let x = rtr.gen_range(0..palace_size);
    let y = rtr.gen_range(0..palace_size);
    (x, y)
}

/// 随机颜色
pub fn color() -> Color {
    let mut rtr = rand::thread_rng();
    let index = rtr.gen_range(0..NUM_COLOR.len());
    NUM_COLOR[index]
}

/// 判断是否是奇数
pub fn odd(n: usize) -> bool {
    n % 2 != 0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_or_four() {
        let all = 100000;
        let nums = (1..=all).fold(Vec::with_capacity(all), |mut nums, _| {
            nums.push(two_or_four());
            nums
        });
        let num_2 = nums.iter().filter(|&&num| num == 2).count();
        println!("num_2 = {} %", num_2 as f64 / all as f64 * 100.0);
        let num_4 = nums.iter().filter(|&&num| num == 4).count();
        println!("num_4 = {} %", num_4 as f64 / all as f64 * 100.0);
    }
}
