# What is this?
This is a library for coloring text.

<img width="1272" alt="vividtext-sample" src="https://github.com/cuticle999/VividText/assets/26399136/5ef8915d-6443-4187-8496-642577e72cf5">

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
    // let gradient_text = apply_gradient(&contents, OutputFormat::Rgb); 
    println!("{}", gradient_text);

    Ok(())
}
```

# license
MIT
