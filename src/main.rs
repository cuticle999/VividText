use std::fs::File;
use std::io::{self, Read};
use gradation::vividtext::{apply_gradient, OutputFormat}; // ここで`OutputFormat`もインポート

fn main() -> io::Result<()> {
    let file_path = "./like.text"; // 実際のファイルパスに置き換えてください
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let gradient_text = apply_gradient(&contents, OutputFormat::Ansi); // ANSI形式の出力が必要な場合
    // または
    // let gradient_text = apply_gradient(&contents, OutputFormat::Rgb); // RGB形式の出力が必要な場合
    println!("{}", gradient_text);

    Ok(())
}