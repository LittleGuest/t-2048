use crate::global::{HISTORY, PALACE_SIZE};
use crate::palace::Game;
use anyhow::Result;
use sled::{self, Db};

lazy_static! {
    static ref DB: Db = sled::open(".T2048/T2048").unwrap();
}

///
pub struct Store;
impl Store {
    /// 插入最高分
    pub fn insert_top_score(score: u128) -> Result<()> {
        let palace_size = unsafe { PALACE_SIZE };
        DB.insert(format!("{}_top_score", palace_size), &score.to_be_bytes())?;
        Ok(())
    }

    /// 获取最高分
    pub fn top_score() -> Result<u128> {
        let palace_size = unsafe { PALACE_SIZE };
        let top_score = DB.get(format!("{}_top_score", palace_size))?;
        if let Some(top_score) = top_score {
            if let Some(&top_score) = slice_as_array!(top_score.as_ref(), [u8; 16]) {
                return Ok(u128::from_be_bytes(top_score));
            }
        }
        Ok(0)
    }

    /// 存历史记录
    pub fn insert_history(game: &Game) -> Result<()> {
        let game_json = serde_json::to_string(game)?;
        DB.insert(HISTORY, game_json.as_bytes())?;
        Ok(())
    }

    /// 获取上次的状态
    pub fn history<'a>() -> Result<Option<Game<'a>>> {
        let history = DB.get(HISTORY)?;
        if let Some(history) = history {
            let game = serde_json::from_slice::<Game>(&*history)?;
            Ok(Some(game))
        } else {
            Ok(None)
        }
    }

    /// 删除
    pub fn remove_history() -> Result<()> {
        DB.remove(HISTORY)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let game = Game::new();

        let json = serde_json::to_string(&game).unwrap();

        DB.insert("history", json.as_bytes()).unwrap();

        let history = DB.get("history").unwrap().unwrap();

        let game = serde_json::from_slice::<Game>(&*history).unwrap();
        println!("game = {:?}", game);

        let top_score = 2048_u128.to_be_bytes();

        DB.insert("top_score", &top_score).unwrap();

        let top_score = DB.get("top_score").unwrap().unwrap();

        let top_score = slice_as_array!(&*top_score, [u8; 16]).unwrap();

        let top_score = u128::from_be_bytes(*top_score);
        println!("top_score = {}", top_score);
    }
}
