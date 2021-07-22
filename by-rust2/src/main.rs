use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("HogeError: {0}")]
    HogeError(u8),
    #[error("FugaError: {0}")]
    FugaError(String),
}

//impl std::fmt::Display for MyError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        use self::MyError::*;
//        match self {
//            HogeError(i) => write!(f, "ほげエラー: {}", i),
//            FugaError(s) => write!(f, "ふがエラー: {}", s),
//        }
//    }
//}
//impl std::error::Error for MyError {}


fn hoge(filename: &str) -> Result<()> {
    let f = std::fs::File::open(filename).context(format!("失敗しました: {}", filename));
    println!("{:?}", f);
    Err(MyError::HogeError(1)).into()?
}

fn main() {
    println!("==============================");
    println!("{:?}", hoge("not-found.txt"));
    println!("==============================");
    println!("{:?}", hoge("sample.txt"));
    println!("==============================");
}
