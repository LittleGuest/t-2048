use std::collections::VecDeque;

use crate::global;
use crate::util;

use global::PALACE_SIZE;
use rand::Rng;
use tui::style::Style;
use tui::widgets::{Borders, Widget};
use tui::{buffer::Buffer, style::Color};
use tui::{layout::Rect, widgets::BorderType};

/// 移动方向
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

/// 数字宫格
pub struct Palace {
    num: u128,
    borders: Borders,
    border_style: Style,
    border_type: BorderType,
}

impl Default for Palace {
    fn default() -> Self {
        Self {
            num: 0,
            borders: Borders::ALL,
            border_style: Style::default(),
            border_type: BorderType::Rounded,
        }
    }
}

impl Palace {
    pub fn num(mut self, num: u128) -> Palace {
        self.num = num;
        self
    }

    // pub fn borders(mut self, borders: Borders) -> Palace {
    //     self.borders = borders;
    //     self
    // }

    // pub fn border_style(mut self, border_style: Style) -> Self {
    //     self.border_style = border_style;
    //     self
    // }

    // pub fn border_type(mut self, border_type: BorderType) -> Self {
    //     self.border_type = border_type;
    //     self
    // }
}

impl Widget for Palace {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut palace_color = global::PALACE_COLOR.lock().unwrap();
        if self.num != 0 && palace_color.contains_key(&self.num) {
            let style;
            if let Some(&c) = palace_color.get(&self.num) {
                style = Style::default().bg(c);
            } else {
                style = self.border_style;
            }

            for y in area.top() + 1..area.bottom() - 1 {
                for x in area.left() + 2..area.right() - 2 {
                    buf.get_mut(x, y).set_style(style);
                }
            }
        } else {
            palace_color.insert(self.num, util::color());
        }

        let symbols = BorderType::line_symbols(self.border_type);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.border_style);
        }

        let text = {
            if self.num == 0 {
                "".to_string()
            } else {
                self.num.to_string()
            }
        };

        buf.set_string(
            area.left() + area.width / 2 - text.len() as u16 / 2,
            area.top() + area.height / 2,
            text,
            Style::default().fg(Color::White),
        );
    }
}

/// 初始化宫格数字
pub fn init_palace() -> Vec<Vec<u128>> {
    let mut palaces = vec![vec![0; PALACE_SIZE]; PALACE_SIZE];
    let mut index = 0;
    loop {
        if index >= 2 {
            break;
        }
        let position = util::position();

        for (x, xp) in palaces.iter_mut().enumerate() {
            for (y, yp) in xp.iter_mut().enumerate() {
                if position.0 == x && position.1 == y {
                    *yp = util::two_or_four();
                    index += 1;
                }
            }
        }
    }

    palaces
}

/// 游戏是否结束
pub fn game_over(palaces: &[Vec<u128>]) -> bool {
    empty_position(palaces).is_empty() && !adjacent_position_can_merge(&palaces)
}

/// 移动
pub fn move_palaces(palaces: &mut Vec<Vec<u128>>, total_score: &mut u128, me: MoveDirection) {
    match me {
        MoveDirection::Up => {
            for x in 0..PALACE_SIZE {
                let mut o = Vec::new();
                for y in 0..PALACE_SIZE {
                    o.push(palaces[y][x]);
                }
                merge(&o, total_score)
                    .iter()
                    .enumerate()
                    .for_each(|(y, yv)| {
                        palaces[y][x] = *yv;
                    });
            }
        }
        MoveDirection::Down => {
            for x in 0..PALACE_SIZE {
                let mut o = Vec::new();
                for y in 0..PALACE_SIZE {
                    o.push(palaces[PALACE_SIZE - y - 1][x]);
                }
                merge(&o.into_iter().collect::<Vec<_>>(), total_score)
                    .iter()
                    .enumerate()
                    .for_each(|(y, yv)| {
                        palaces[PALACE_SIZE - y - 1][x] = *yv;
                    });
            }
        }
        MoveDirection::Left => {
            (0..PALACE_SIZE).for_each(|i| {
                palaces[i] = merge(&palaces[i], total_score);
            });
        }
        MoveDirection::Right => {
            (0..PALACE_SIZE).for_each(|i| {
                palaces[i] = merge(
                    &palaces[i].iter().rev().cloned().collect::<Vec<_>>(),
                    total_score,
                )
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>();
            });
        }
    }

    empty_position_gen(palaces);
}

/// 在宫格剩余空位置上生成方块
pub fn empty_position_gen(palaces: &mut Vec<Vec<u128>>) {
    let palace_empty = empty_position(&palaces);

    if palace_empty.is_empty() {
        return;
    }

    // 如果有空位置，在空位置随机生成一个数字方块
    let position_at_index = rand::thread_rng().gen_range(0..palace_empty.len());
    let (position_x, position_y) = palace_empty[position_at_index];
    palaces[position_x][position_y] = util::two_or_four();
}

/// 返回空位置坐标
fn empty_position(palaces: &[Vec<u128>]) -> Vec<(usize, usize)> {
    palaces
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut palace_empty, (x, xp)| {
            xp.iter().enumerate().for_each(|(y, yp)| {
                if *yp == 0 {
                    palace_empty.push((x, y));
                }
            });
            palace_empty
        })
}

/// 判断是否有相邻位置能否合并的
fn adjacent_position_can_merge(palaces: &[Vec<u128>]) -> bool {
    for (x, xp) in palaces.iter().enumerate() {
        for (y, _) in xp.iter().enumerate() {
            if y == 0 {
                continue;
            }
            if palaces[x][y] == palaces[x][y - 1] {
                return true;
            }
        }
    }

    for (x, xp) in palaces.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (y, _) in xp.iter().enumerate() {
            if palaces[x][y] == palaces[x - 1][y] {
                return true;
            }
        }
    }

    false
}

/// 向左合并数字
fn merge(vs: &[u128], score: &mut u128) -> Vec<u128> {
    let mut q = vs.iter().collect::<VecDeque<_>>();
    // 存放合并之后的值的队列
    let mut cvq = VecDeque::new();
    // 是否合并过
    let mut merged = false;
    while !q.is_empty() {
        let qv = q.pop_front().unwrap();
        if *qv != 0 {
            if let Some(cv) = cvq.back() {
                if *cv == *qv && !merged {
                    cvq.pop_back();
                    cvq.push_back(*qv * 2);
                    merged = true;

                    *score += *qv * 2;
                } else {
                    cvq.push_back(*qv);
                    merged = false;
                }
            } else {
                cvq.push_back(*qv);
                merged = false;
            }
        }
    }

    // 将剩余的补0
    if cvq.len() < PALACE_SIZE {
        (0..PALACE_SIZE - cvq.len()).for_each(|_| {
            cvq.push_back(0);
        });
    }

    cvq.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    
    
    // #[test]
    // fn test_merge() {
    //     // 向左
    //     let mut palaces = vec![
    //         vec![0, 0, 0, 2],
    //         vec![4, 2, 0, 2],
    //         vec![0, 2, 2, 0],
    //         vec![0, 0, 0, 0],
    //     ];

    //     (0..PALACE_SIZE).for_each(|i| {
    //         palaces[i] = merge(&palaces[i]);
    //     });

    //     assert_eq!(
    //         vec![
    //             vec![2, 0, 0, 0],
    //             vec![4, 4, 0, 0],
    //             vec![4, 0, 0, 0],
    //             vec![0, 0, 0, 0],
    //         ],
    //         palaces
    //     );

    //     // 向右
    //     let mut palaces = vec![
    //         vec![0, 0, 0, 2],
    //         vec![4, 2, 0, 2],
    //         vec![0, 2, 2, 0],
    //         vec![0, 0, 0, 0],
    //     ];

    //     (0..PALACE_SIZE).for_each(|i| {
    //         palaces[i] = merge(&palaces[i].iter().rev().cloned().collect::<Vec<_>>())
    //             .iter()
    //             .rev()
    //             .cloned()
    //             .collect::<Vec<_>>();
    //     });

    //     assert_eq!(
    //         vec![
    //             vec![0, 0, 0, 2],
    //             vec![0, 0, 4, 4],
    //             vec![0, 0, 0, 4],
    //             vec![0, 0, 0, 0],
    //         ],
    //         palaces
    //     );

    //     // 向上
    //     let mut palaces = vec![
    //         vec![0, 0, 0, 2],
    //         vec![4, 2, 0, 2],
    //         vec![0, 2, 2, 0],
    //         vec![0, 0, 0, 0],
    //     ];

    //     for x in 0..PALACE_SIZE {
    //         let mut o = Vec::new();
    //         for y in 0..PALACE_SIZE {
    //             o.push(palaces[y][x]);
    //         }
    //         merge(&o).iter().enumerate().for_each(|(y, yv)| {
    //             palaces[y][x] = *yv;
    //         });
    //     }

    //     assert_eq!(
    //         vec![
    //             vec![4, 4, 2, 4],
    //             vec![0, 0, 0, 0],
    //             vec![0, 0, 0, 0],
    //             vec![0, 0, 0, 0],
    //         ],
    //         palaces
    //     );

    //     // 向下
    //     let mut palaces = vec![
    //         vec![0, 0, 0, 2],
    //         vec![4, 2, 0, 2],
    //         vec![0, 2, 2, 0],
    //         vec![0, 0, 0, 0],
    //     ];

    //     for x in 0..PALACE_SIZE {
    //         let mut o = Vec::new();
    //         for y in 0..PALACE_SIZE {
    //             o.push(palaces[PALACE_SIZE - y - 1][x]);
    //         }
    //         merge(&o.into_iter().collect::<Vec<_>>())
    //             .iter()
    //             .enumerate()
    //             .for_each(|(y, yv)| {
    //                 palaces[PALACE_SIZE - y - 1][x] = *yv;
    //             });
    //     }

    //     assert_eq!(
    //         vec![
    //             vec![0, 0, 0, 0],
    //             vec![0, 0, 0, 0],
    //             vec![0, 0, 0, 0],
    //             vec![4, 4, 2, 4],
    //         ],
    //         palaces
    //     );
    // }
}
