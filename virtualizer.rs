use shiplift::Docker;
use tauri::Manager;
use serde::Serialize;
use async_std::task;

#[derive(Serialize)]
struct VirtualizationSuggestion {
    name: String,
    reason: String,
}

pub fn check_docker(app_handle: tauri::AppHandle) {
    task::block_on(async {
        let docker = Docker::new();
        match docker.containers().list(&Default::default()).await {
            Ok(containers) => {
                let mut suggestions = Vec::new();
                for container in containers {
                    suggestions.push(VirtualizationSuggestion {
                        name: container.names.get(0).unwrap_or(&"Unnamed".to_string()).clone(),
                        reason: "Running container".to_string(),
                    });
                }
                app_handle.emit_all("virtualization_suggestions", suggestions).unwrap();
            }
            Err(e) => eprintln!("Docker error: {}", e),
        }
    });
}