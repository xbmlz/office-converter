use std::{path::Path, process::Command};

use crate::utils::{find_office_home, format_path};

const UNI_SCRIPT: &'static str = include_str!("unoconv");

pub struct ConverterConfig {
    home: String,
    host: String,
    port: u16,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            home: find_office_home().unwrap(),
            host: "127.0.0.1".to_string(),
            port: 2002,
        }
    }
}

pub struct Converter {
    config: ConverterConfig,
}

impl Converter {
    pub fn new() -> Self {
        Self::new_with_config(ConverterConfig::default())
    }

    pub fn new_with_config(config: ConverterConfig) -> Self {
        Self { config }
    }

    pub fn convert(&self, input: &str, output: &str) {
        let in_path = format_path(input);
        let out_path = format_path(output);

        let args = &[
            "-c",
            UNI_SCRIPT,
            in_path.as_str(),
            "-O",
            out_path.as_str(),
            "-H",
            self.config.host.as_str(),
            "-P",
            &format!("{}", self.config.port),
        ];

        let program = Path::new(&self.config.home).join("program").join("python");

        println!("{:?}", program);

        let out = Command::new(program).args(args).output().unwrap();

        if !out.status.success() {
            panic!(
                "conversion failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Converter, OfficeManager};

    #[test]
    fn converter_test() {
        let mut om = OfficeManager::new();

        om.start();

        let con = Converter::new();

        con.convert("E:/test.pptx", "E:/test.pdf");

        om.stop()
    }
}
