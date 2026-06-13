// air_client/src/virtual_display/kde.rs
use super::Result;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub struct VirtualDisplay {
    process: Option<std::process::Child>,
    output_name: String,
    #[allow(unused)]
    name: String,
    width: u32,
    height: u32,
    refresh_rate: u32,
}

impl std::fmt::Display for VirtualDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}x{}@{}",
            self.output_name, self.width, self.height, self.refresh_rate
        )
    }
}

impl VirtualDisplay {
    pub fn create(width: u32, height: u32) -> Result<Self> {
        let name = "air-virtual";
        let output_name = format!("Virtual-{}", name);
        let password = "temp123"; // Временный пароль, нам не важен

        // 1. Запускаем krfb-virtualmonitor
        let process = Command::new("krfb-virtualmonitor")
            .args([
                "--resolution",
                &format!("{}x{}", width, height),
                "--name",
                name,
                "--password",
                password,
                "--port",
                "5900",
            ])
            .spawn()?;

        sleep(Duration::from_millis(300));

        Ok(Self {
            process: Some(process),
            output_name,
            name: name.to_string(),
            width,
            height,
            refresh_rate: 60,
        })
    }

    pub fn output_name(&self) -> &str {
        &self.output_name
    }

    pub fn remove(self) {
        if let Some(mut process) = self.process {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}
