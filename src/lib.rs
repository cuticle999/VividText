pub mod lolcat {
    pub enum OutputFormat {
        Ansi, // ANSIエスケープコードを使用して8ビットカラーを適用
        EightBit, // 直接8ビットカラーコードを使用（ターミナルでの表示に適している）
    }

    pub fn apply_gradient(text: &str, format: OutputFormat) -> String {
        let mut result = String::new();
        let hue_step = 360.0 / text.chars().count() as f32;

        for (i, char) in text.chars().enumerate() {
            let hue = (i as f32) * hue_step;
            // 8ビットカラーインデックスを計算
            let base = 16; // 基本色とグレースケールを除外
            let color_index = base + ((hue / 360.0) * 216.0).floor() as u8;

            // ANSIエスケープコードまたは直接8ビットカラーコードの使用
            let color_code = match format {
                OutputFormat::Ansi | OutputFormat::EightBit => format!("\x1b[38;5;{}m", color_index),
            };

            result.push_str(&color_code);
            result.push(char); // グラデーションを適用した文字を追加
        }
        result.push_str("\x1b[0m"); // テキストの終わりにリセットコードを追加して、後続のテキストに影響を与えないようにする
        result
    }
}