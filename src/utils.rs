use std::{
    env,
    path::{self, Path, PathBuf},
};

pub fn format_path(input_path: &str) -> String {
    let path = Path::new(input_path);
    if path.is_relative() {
        if let Some(current_dir) = std::env::current_dir().ok() {
            let absolute_path = current_dir
                .join(path)
                .canonicalize()
                .unwrap_or(PathBuf::from(""));
            return absolute_path.to_string_lossy().to_string();
        }
    }
    input_path.to_string()
}

pub fn find_office_home() -> Option<String> {
    // 1. find env "office.home"
    if let Ok(home) = env::var("office.home") {
        return Some(home);
    }
    // 2. find dir
    let os = env::consts::OS;

    match os {
        "windows" => {
            let program_files64 =
                env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
            let program_files32 = env::var("C:\\Program Files(x86)")
                .unwrap_or_else(|_| "C:\\Program Foiles (x86)".to_string());
            find_path(
                "program\\soffice.exe",
                &[
                    &format!("{}\\{}", program_files64, "LibreOffice"),
                    &format!("{}\\{}", program_files64, "LibreOffice 5"),
                    &format!("{}\\{}", program_files32, "LibreOffice 5"),
                    &format!("{}\\{}", program_files32, "OpenOffice 4"),
                    &format!("{}\\{}", program_files64, "LibreOffice 4"),
                    &format!("{}\\{}", program_files32, "LibreOffice 4"),
                    &format!("{}\\{}", program_files64, "LibreOffice 3"),
                    &format!("{}\\{}", program_files32, "LibreOffice 3"),
                ],
            )
        }
        "darwin" => find_path(
            "program/soffice",
            &[
                "/Applications/LibreOffice.app/Contents",
                "/Applications/OpenOffice.app/Contents",
                "/Applications/OpenOffice.org.app/Contents",
            ],
        ),
        "linux" => find_path(
            "program/soffice.bin",
            &[
                "/usr/lib/libreoffice",
                "/usr/local/lib64/libreoffice",
                "/usr/local/lib/libreoffice",
                "/opt/libreoffice",
                "/usr/lib64/openoffice",
                "/usr/lib64/openoffice.org3",
                "/usr/lib64/openoffice.org",
                "/usr/lib/openoffice",
                "/usr/lib/openoffice.org3",
                "/usr/lib/openoffice.org",
                "/opt/openoffice4",
                "/opt/openoffice.org3",
            ],
        ),
        _ => None,
    }
}

pub fn find_office_executable(home: &str) -> Option<String> {
    let path = match env::consts::OS {
        "windows" => format!("{}/program/soffice.exe", home),
        "macos" => format!("{}/program/soffice.bin", home),
        "linux" => format!("{}/program/soffice", home),
        _ => return None,
    };

    Some(path)
}

pub fn find_path(bin: &str, paths: &[&str]) -> Option<String> {
    paths.into_iter().find_map(|path| {
        let path = path::Path::new(path);
        if path.join(bin).exists() {
            return Some(path.to_str().unwrap().to_string());
        }
        None
    })
}
