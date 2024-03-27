// use std::fs::File;
// use std::io::{self, Read};
// use vividtext::colorful_text::{apply_gradient, OutputFormat}; // ここで`OutputFormat`もインポート

// fn main() -> io::Result<()> {
//     let file_path = "./like.text"; // 実際のファイルパスに置き換えてください
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let gradient_text = apply_gradient(&contents, OutputFormat::Ansi); // ANSI形式の出力が必要な場合
//     // または
//     // let gradient_text = apply_gradient(&contents, OutputFormat::Rgb); // RGB形式の出力が必要な場合
//     println!("{}", gradient_text);

//     Ok(())
// }

// `colorful_text`モジュールとその内容を`lib.rs`から直接インポートするための`use`文を追加
// `vividtext`クレートから`colorful_text`モジュールの内容をインポート

// 指定したテキストを表示するためのコード例
use vividtext::colorful_text::{apply_gradient, OutputFormat};

fn main() {
    let text = "Hello, world!";
    // 修正したインポートを使用
    let formatted_text = apply_gradient(text, OutputFormat::Ansi);
    println!("{}", formatted_text);
}

