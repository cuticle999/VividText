# What is this?
This is a library for coloring text.
At this stage, it is designed for output on the CLI.

<img width="1270" alt="vividtext-cli" src="https://github.com/cuticle999/VividText/assets/26399136/8c60fa61-0a4a-4f83-9263-290e51e03414">


# How to use
Add a dependency
```cargo add vividtext```

Example of coloring the specified text
main.rs
```
use vividtext::colorful_text::{apply_gradient, OutputFormat};

fn main() {
    let text = "Hello, world!";
    let formatted_text = apply_gradient(text, OutputFormat::Ansi);
    println!("{}", formatted_text);
}
```

Example of outputting text entered in a text file
main.rs
```
use std::fs::File;
use std::io::{self, Read};
use vividtext::colorful_text::{apply_gradient, OutputFormat};

fn main() -> io::Result<()> {
    let file_path = "./like.text";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let gradient_text = apply_gradient(&contents, OutputFormat::Ansi);
    // または
    // let gradient_text = apply_gradient(&contents, OutputFormat::Rgb); 
    println!("{}", gradient_text);

    Ok(())
}
```

How to use with CLI
If you specify a text file, etc., the text will be colored on the CLI.
```cargo install vividtext ```

```vividtext your_file```

# license
MIT
