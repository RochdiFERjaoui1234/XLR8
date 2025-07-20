use yara::{Compiler, Rules};
use sysinfo::{System, SystemExt, ProcessExt};
use tauri::Manager;
use serde::Serialize;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
struct MalwareAlert {
    pid: u32,
    name: String,
    rule: String,
}

pub fn start_scanning(app_handle: tauri::AppHandle) {
    let rules_str = r#"
        rule suspicious_process {
            strings:
                $malware1 = "malicious" nocase
                $malware2 = "backdoor" nocase
            condition:
                $malware1 or $malware2
        }
    "#;
    let mut compiler = Compiler::new().unwrap();
    compiler.add_rules_str(rules_str).unwrap();
    let rules = compiler.compile_rules().unwrap();

    let mut system = System::new_all();
    loop {
        system.refresh_all();
        let mut alerts = Vec::new();

        for (pid, process) in system.processes() {
            if let Some(exe_path) = process.exe() {
                if let Ok(mut file) = File::open(exe_path) {
                    let mut buffer = Vec::new();
                    if file.read_to_end(&mut buffer).is_ok() {
                        if let Ok(results) = rules.scan_mem(&buffer, 5) {
                            for result in results {
                                alerts.push(MalwareAlert {
                                    pid: *pid as u32,
                                    name: process.name().to_string(),
                                    rule: result.identifier.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        app_handle.emit_all("malware_alerts", &alerts).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}