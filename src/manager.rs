use std::{
    net::TcpStream,
    process::{self, Command, Stdio},
    time::Duration,
};

use crate::utils::{find_office_executable, find_office_home};

pub struct OfficeConfig {
    home: String,
    host: String,
    port: u16,
    timeout: u64,
}

impl Default for OfficeConfig {
    fn default() -> Self {
        Self {
            home: find_office_executable(find_office_home().unwrap().as_str()).unwrap(),
            host: "127.0.0.1".to_string(),
            port: 2002,
            timeout: 5000,
        }
    }
}

pub struct OfficeManager {
    process: Option<process::Child>,
    config: OfficeConfig,
}

impl OfficeManager {
    pub fn new() -> Self {
        Self::new_with_config(Default::default())
    }

    pub fn new_with_config(config: OfficeConfig) -> Self {
        Self {
            process: None,
            config,
        }
    }

    pub fn start(&mut self) {
        if self.config.home.is_empty() {
            panic!("Office home not found");
        }
        let home = self.config.home.clone();
        let host = self.config.host.clone();
        let port = self.config.port;
        let timeout = Duration::from_millis(self.config.timeout);

        let args = [
            "--headless",
            "--invisible",
            "--nocrashreport",
            "--nodefault",
            "--nologo",
            "--nofirststartwizard",
            "--norestore",
            &format!("--accept=socket,host={host},port={port},tcpNoDelay=1;urp;StarOffice.ComponentContext"),
        ];

        println!("Starting office process with args: {:?}", args);

        let child = Command::new(home)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start office process");
        self.process = Some(child);

        let bind_addr = format!("{}:{}", host, port)
            .parse()
            .expect("Invalid bind address");

        // 在超时时间内等待连接成功
        if let Ok(_stream) = TcpStream::connect_timeout(&bind_addr, timeout) {
            println!("Connected to the server!");
        } else {
            println!("Couldn't connect to server...");
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            child.kill().expect("Failed to kill office process");
        }
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::OfficeManager;

    #[test]
    fn manager_test() {
        let mut om = OfficeManager::new();

        om.start();

        assert!(om.is_running());

        om.stop()
    }
}
