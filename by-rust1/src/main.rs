
#[derive(Debug)]
enum MyError {
    HogeError(u8),
    FugaError(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::MyError::*;
        match self {
            HogeError(i) => write!(f, "ほげエラー: {}", i),
            FugaError(s) => write!(f, "ふがエラー: {}", s),
        }
    }
}
impl std::error::Error for MyError {}

fn hoge(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filename)?;
    //let mut contents = String::new();
    //f.read_to_string(&mut contents)?;
    println!("{:?}", f);
    Err(Box::new(MyError::HogeError(1)))
}

fn main() {
    println!("==============================");
    println!("{:?}", hoge("not-found.txt"));
    println!("==============================");
    println!("{:?}", hoge("sample.txt"));
    println!("==============================");
}
