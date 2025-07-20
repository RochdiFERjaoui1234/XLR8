use sysinfo::{System, SystemExt, ProcessExt};
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::ensemble::isolation_forest::IsolationForest;
use tauri::Manager;
use serde::Serialize;

#[derive(Serialize)]
struct OptimizationSuggestion {
    pid: u32,
    name: String,
    reason: String,
}

pub fn start_monitoring(app_handle: tauri::AppHandle) {
    let mut system = System::new_all();
    system.refresh_all();

    // Collect baseline data for training
    let mut baseline_data = Vec::new();
    for (_, process) in system.processes() {
        let cpu = process.cpu_usage();
        let memory = process.memory();
        baseline_data.push(vec![cpu as f64, memory as f64 / 1_000_000.0]);
    }
    let baseline_matrix = DenseMatrix::from_2d_vec(&baseline_data);
    let forest = IsolationForest::fit(&baseline_matrix, Default::default()).unwrap();

    // Monitoring loop
    loop {
        system.refresh_all();
        let mut data = Vec::new();
        let mut suggestions = Vec::new();

        for (pid, process) in system.processes() {
            let cpu = process.cpu_usage();
            let memory = process.memory();
            data.push(vec![cpu as f64, memory as f64 / 1_000_000.0]);
            if cpu > 50.0 || memory > 100_000_000 {
                suggestions.push(OptimizationSuggestion {
                    pid: *pid as u32,
                    name: process.name().to_string(),
                    reason: "High resource usage".to_string(),
                });
            }
        }

        let matrix = DenseMatrix::from_2d_vec(&data);
        let predictions = forest.predict(&matrix).unwrap();
        for (i, &score) in predictions.iter().enumerate() {
            if score > 0.5 {
                let pid = system.processes().keys().nth(i).unwrap();
                let process = system.processes().get(pid).unwrap();
                suggestions.push(OptimizationSuggestion {
                    pid: *pid as u32,
                    name: process.name().to_string(),
                    reason: "Anomalous resource usage".to_string(),
                });
            }
        }

        app_handle.emit_all("optimization_suggestions", &suggestions).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}