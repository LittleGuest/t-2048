use std::collections::HashMap;

use crate::util;
use std::sync::Arc;
use std::sync::Mutex;
use tui::style::Color;

/// 游戏说明
pub const GAME_DESCRIPTION: &str = r#"
1. 通过控制
  方向键上下左右
  或
  h(左)j(下)k(上)l(右)
  移动数字方块；
2. 重新开始(r)
   撤回(z)
   退出(q | Esc)
"#;

/// 宫格初始大小 n*n
pub const PALACE_SIZE: usize = 4;

lazy_static! {
    /// 宫格数字
    // pub static ref PALACE_NUM: Arc<Mutex<Vec<Vec<u128>>>> =  Arc::new(Mutex::new(vec![vec![0; PALACE_SIZE]; PALACE_SIZE]));
    /// 总分数
    // pub static ref TOTAL_SCORE:Arc<Mutex<u128>> = Arc::new(Mutex::new(0));
    /// 颜色数组
    pub static ref NUM_COLOR: Vec<Color> = {
        let mut num_color = Vec::new();
        num_color.push(Color::Rgb(127, 156, 138));
        num_color.push(Color::Rgb(58, 233, 102));
        num_color.push(Color::Rgb(158, 88, 105));
        num_color.push(Color::Rgb(228, 101, 39));
        num_color.push(Color::Rgb(211,84,0));
        num_color.push(Color::Rgb(233, 121, 58));
        num_color.push(Color::Rgb(13, 141, 72));
        num_color.push(Color::Rgb(78, 110, 227));
        num_color.push(Color::Rgb(175, 132, 247));
        num_color.push(Color::Rgb(255, 68, 26));
        num_color.push(Color::Rgb(225, 11, 43));
        num_color.push(Color::Rgb(232, 139, 0));
        num_color.push(Color::Rgb(255, 176, 97));
        num_color.push(Color::Rgb(255, 123, 36));
        num_color.push(Color::Rgb(2, 179, 64));
        num_color.push(Color::Rgb(115, 46, 126));
        num_color
    };
    /// 数字对应的颜色
    pub static ref PALACE_COLOR: Arc<Mutex<HashMap<u128, Color>>> = {
        let mut num_color_map = HashMap::new();
        (1..=16).for_each(|i|{
            num_color_map.insert(2_u128.pow(i), util::color());
        });
        Arc::new(Mutex::new(num_color_map))
    };
}
