# Office Converter

Convert between any document format supported by LibreOffice/OpenOffice.

## Requirements

- [Apache OpenOffice](https://www.openoffice.org/) or [LibreOffice](https://www.libreoffice.org/); the latest stable version is usually recommended.

## Usage

To use `office-converter`, add this to your `Cargo.toml`:


```toml
[dependencies]
office-converter = "0.1.0"
```

Then, on your `main.rs`

```rust
use office_converter::OfficeConverter;

fn main() {

    let mut om = manager::OfficeManager::new();

    om.start();

    let con = converter::Converter::new();

    con.convert("E:/test.pptx", "E:/test.pdf");

    om.stop()
}