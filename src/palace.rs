use std::collections::VecDeque;

use anyhow::Result;
use global::PALACE_SIZE;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tui::widgets::{Borders, Widget};
use tui::{buffer::Buffer, style::Color};
use tui::{layout::Rect, widgets::BorderType};
use tui::{style::Style, widgets::ListState};

use crate::global;
use crate::store::Store;
use crate::util;

/// 移动方向
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

///
#[derive(Debug, Serialize, Deserialize)]
pub struct Game<'a> {
    /// 宫格数字
    pub palaces: Vec<Vec<u128>>,
    /// 总分
    pub total_score: u128,
    /// 最高分
    pub top_score: u128,
    /// 移动步数
    pub move_steps: u128,
    /// 结束    
    pub game_over: bool,
    /// 模式
    #[serde(skip)]
    pub model: Model<'a>,
}

/// n * n 模式
#[derive(Debug, Default)]
pub struct Model<'a> {
    pub state: ListState,
    pub items: Vec<(&'a str, usize)>,
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        let mut ls = ListState::default();
        ls.select(Some(1));

        Self {
            state: ls,
            items: vec![("3 * 3", 3), ("4 * 4", 4), ("5 * 5", 5), ("6 * 6", 6)],
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        unsafe {
            PALACE_SIZE = self.items[i].1;
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        unsafe {
            PALACE_SIZE = self.items[i].1;
        }
    }
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        Self {
            palaces: Self::init_palace(),
            total_score: 0,
            top_score: Self::top_score(),
            move_steps: 0,
            game_over: false,
            model: Model::new(),
        }
    }

    /// 初始化宫格数字
    fn init_palace() -> Vec<Vec<u128>> {
        let mut palaces = unsafe { vec![vec![0; PALACE_SIZE]; PALACE_SIZE] };
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

    /// 改变模式
    pub fn change_model(&mut self) {
        self.palaces = Self::init_palace();
        self.total_score = 0;
        self.top_score = Self::top_score();
        self.move_steps = 0;
        self.game_over = false;
    }

    /// 游戏是否结束
    pub fn game_over(&self) -> bool {
        self.empty_position().is_empty() && !self.adjacent_position_can_merge()
    }

    /// 返回空位置坐标
    fn empty_position(&self) -> Vec<(usize, usize)> {
        self.palaces
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
    fn adjacent_position_can_merge(&self) -> bool {
        for (x, xp) in self.palaces.iter().enumerate() {
            for (y, _) in xp.iter().enumerate() {
                if y == 0 {
                    continue;
                }
                if self.palaces[x][y] == self.palaces[x][y - 1] {
                    return true;
                }
            }
        }

        for (x, xp) in self.palaces.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (y, _) in xp.iter().enumerate() {
                if self.palaces[x][y] == self.palaces[x - 1][y] {
                    return true;
                }
            }
        }

        false
    }

    /// 移动
    pub fn move_palaces(&mut self, md: MoveDirection) {
        let palace_size = unsafe { PALACE_SIZE };
        match md {
            MoveDirection::Up => {
                for x in 0..palace_size {
                    let mut o = Vec::new();
                    for y in 0..palace_size {
                        o.push(self.palaces[y][x]);
                    }
                    self.merge(&o).iter().enumerate().for_each(|(y, yv)| {
                        self.palaces[y][x] = *yv;
                    });
                }
            }
            MoveDirection::Down => {
                for x in 0..palace_size {
                    let mut o = Vec::new();
                    for y in 0..palace_size {
                        o.push(self.palaces[palace_size - y - 1][x]);
                    }
                    self.merge(&o.into_iter().collect::<Vec<_>>())
                        .iter()
                        .enumerate()
                        .for_each(|(y, yv)| {
                            self.palaces[palace_size - y - 1][x] = *yv;
                        });
                }
            }
            MoveDirection::Left => {
                (0..palace_size).for_each(|i| {
                    self.palaces[i] = self.merge(&self.palaces[i].clone());
                });
            }
            MoveDirection::Right => {
                (0..palace_size).for_each(|i| {
                    self.palaces[i] = self
                        .merge(&self.palaces[i].iter().rev().cloned().collect::<Vec<_>>())
                        .iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<_>>();
                });
            }
        }

        self.empty_position_gen();
    }

    /// 在宫格剩余空位置上生成方块
    fn empty_position_gen(&mut self) {
        let palace_empty = self.empty_position();

        if palace_empty.is_empty() {
            return;
        }

        // 如果有空位置，在空位置随机生成一个数字方块
        let position_at_index = rand::thread_rng().gen_range(0..palace_empty.len());
        let (position_x, position_y) = palace_empty[position_at_index];
        self.palaces[position_x][position_y] = util::two_or_four();
    }

    /// 向左合并数字
    fn merge(&mut self, vs: &[u128]) -> Vec<u128> {
        let palace_size = unsafe { PALACE_SIZE };
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

                        self.total_score += *qv * 2;
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
        if cvq.len() < palace_size {
            (0..palace_size - cvq.len()).for_each(|_| {
                cvq.push_back(0);
            });
        }

        cvq.into_iter().collect::<Vec<_>>()
    }

    /// 最高分
    pub fn top_score() -> u128 {
        Store::top_score().unwrap_or_default()
    }

    /// 最高分
    pub fn insert_top_score(&mut self) -> Result<()> {
        if self.total_score > self.top_score {
            self.top_score = self.total_score;
            let _ = Store::insert_top_score(self.total_score);
        }
        Ok(())
    }

    /// 记录移动之前的状态
    pub fn insert_history(&self) -> Result<()> {
        Store::insert_history(self)?;
        Ok(())
    }

    /// 撤回
    pub fn back(&mut self) -> Result<()> {
        if let Some(history) = Store::history()? {
            self.palaces = history.palaces;
            self.total_score = history.total_score;
            self.top_score = Self::top_score();
            self.move_steps = history.move_steps;
            self.game_over = history.game_over;

            Store::remove_history()?;
        }
        Ok(())
    }
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
            area.left() + area.width / 2 - text.chars().count() as u16 / 2,
            area.top() + area.height / 2,
            text,
            Style::default().fg(Color::White),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let mut game = Game::new();

        // 向左
        let mut palaces = vec![
            vec![0, 0, 0, 2],
            vec![4, 2, 0, 2],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
        ];

        (0..4).for_each(|i| {
            palaces[i] = game.merge(&palaces[i]);
        });

        assert_eq!(
            vec![
                vec![2, 0, 0, 0],
                vec![4, 4, 0, 0],
                vec![4, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            palaces
        );

        // 向右
        let mut palaces = vec![
            vec![0, 0, 0, 2],
            vec![4, 2, 0, 2],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
        ];

        (0..4).for_each(|i| {
            palaces[i] = game
                .merge(&palaces[i].iter().rev().cloned().collect::<Vec<_>>())
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>();
        });

        assert_eq!(
            vec![
                vec![0, 0, 0, 2],
                vec![0, 0, 4, 4],
                vec![0, 0, 0, 4],
                vec![0, 0, 0, 0],
            ],
            palaces
        );

        // 向上
        let mut palaces = vec![
            vec![0, 0, 0, 2],
            vec![4, 2, 0, 2],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
        ];

        for x in 0..4 {
            let mut o = Vec::new();
            for y in 0..4 {
                o.push(palaces[y][x]);
            }
            game.merge(&o).iter().enumerate().for_each(|(y, yv)| {
                palaces[y][x] = *yv;
            });
        }

        assert_eq!(
            vec![
                vec![4, 4, 2, 4],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            palaces
        );

        // 向下
        let mut palaces = vec![
            vec![0, 0, 0, 2],
            vec![4, 2, 0, 2],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
        ];

        for x in 0..4 {
            let mut o = Vec::new();
            for y in 0..4 {
                o.push(palaces[4 - y - 1][x]);
            }
            game.merge(&o.into_iter().collect::<Vec<_>>())
                .iter()
                .enumerate()
                .for_each(|(y, yv)| {
                    palaces[4 - y - 1][x] = *yv;
                });
        }

        assert_eq!(
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![4, 4, 2, 4],
            ],
            palaces
        );
    }
}
