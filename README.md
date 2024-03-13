# Office Converter

Convert between any document format supported by LibreOffice/OpenOffice.

## Features

- Using LibreOffice listener mode -  This lowers the CPU load when converting many documents with somewhere between 50% and 75%
- Auto-detecting file format
- Supports all document formats supported by LibreOffice/OpenOffice

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
use office_converter::{OfficeManager, OfficeConverter};

fn main() {

    let mut om = OfficeManager::new();

    om.start();

    let con = Converter::new();

    con.convert("E:/test.pptx", "E:/test.pdf");

    om.stop()
}