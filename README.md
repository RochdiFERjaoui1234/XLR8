XLR8 - Windows System Optimizer
 
XLR8 is a powerful, Rust-based application designed to optimize Windows performance, enhance system security, and manage resources intelligently. Built with Tauri for a lightweight, web-based interface, XLR8 monitors processes, uses machine learning to identify optimization opportunities, scans for malware, and supports virtualization via Docker. Whether you're a gamer, developer, or everyday user, XLR8 helps keep your system running smoothly.
Repository: https://github.com/RochdiFERjaoui1234/XLR8
Features

Process Monitoring: Tracks CPU, memory, and process details in real-time.
Smart Optimization: Uses machine learning (Isolation Forest) to detect resource-heavy or anomalous processes and suggest actionsplots to freeze or virtualize.
Security Scanning: Identifies suspicious processes using YARA rules and alerts users.
Virtualization Support: Integrates with Docker to isolate processes in containers for better resource management.
Turbo Mode: Automatically freezes high-memory or suspicious processes for a quick performance boost.
User-Friendly UI: A clean, browser-based dashboard displays process data, optimization suggestions, malware alerts, and virtualization options.

Prerequisites
To build and run XLR8, you need:

Rust: Latest stable version (install via rustup).
Tauri Dependencies:
Windows: WebView2 (included in Windows 10/11).
Node.js and npm for frontend build (npm install).


Docker: Required for virtualization features (install Docker Desktop).
Optional: Python 3.8+ with PyTorch for advanced ML model support (if extending ML capabilities).
OS: Windows 10 or 11 (64-bit).

Installation

Clone the Repository:
git clone https://github.com/RochdiFERjaoui1234/XLR8.git
cd XLR8


Install Dependencies:
cargo install tauri-cli
npm install


Build the Application:
cargo tauri build

This generates an executable in target/release/xlr8.exe.

Run the Application:
cargo tauri dev

Or launch target/release/xlr8.exe directly.


Usage

Launch XLR8:

Run via cargo tauri dev or double-click xlr8.exe.
The dashboard opens, showing four sections:
Processes: Lists running processes (PID, name, memory, status).
Optimization Suggestions: Flags processes using >50% CPU or >100MB memory.
Malware Alerts: Shows potential security threats detected by YARA.
Virtualization Suggestions: Lists Docker containers or processes for isolation.




Interact with the Dashboard:

Processes: View real-time data. Click Freeze to suspend a process or Resume to restore it.
Optimization Suggestions: Review and act on suggestions to freeze or virtualize high-resource processes.
Malware Alerts: Investigate flagged processes and freeze if needed.
Virtualization Suggestions: Isolate processes in Docker containers for efficiency.
Turbo Mode: Click Enable Turbo Mode to auto-freeze high-memory or suspicious processes.


Example:

Open XLR8 and spot "chrome.exe" in "Optimization Suggestions" using 150MB.
Click Freeze to pause it, freeing resources.
Check "Malware Alerts" for suspicious processes and take action.
Use "Virtualization Suggestions" to run heavy processes in Docker.



Project Structure

src/main.rs: Core logic, process monitoring, and Tauri setup.
src/ml_optimizer.rs: ML-based optimization using Isolation Forest.
src/security.rs: Malware scanning with YARA rules.
src/virtualizer.rs: Docker integration for process isolation.
src/index.html: Web-based UI for the dashboard.
Cargo.toml: Dependencies (tauri, winapi, sysinfo, smartcore, yara, shiplift).

Dependencies

tauri: Desktop app framework.
winapi: Windows process management.
sysinfo: System resource monitoring.
smartcore: Machine learning (Isolation Forest).
yara: Malware detection.
shiplift: Docker integration.
serde: Data serialization.
async-std: Asynchronous runtime for Docker.

Troubleshooting

UI Fails to Load: Ensure WebView2 is installed and run npm install.
Docker Errors: Verify Docker Desktop is running and accessible.
High CPU Usage: Increase monitoring intervals in ml_optimizer.rs (e.g., 10s to 30s).
No Malware Alerts: Update YARA rules in security.rs with comprehensive patterns.

Contributing
We welcome contributions! To get started:

Fork the repository.
Create a feature branch (git checkout -b feature/your-feature).
Commit changes (git commit -m "Add your feature").
Push to your fork (git push origin feature/your-feature).
Open a pull request.

Please follow the Code of Conduct and ensure code is formatted with cargo fmt.
License
Licensed under the Appache 2.0 License. See LICENSE for details.
Contact
For issues, suggestions, or questions, open an issue on GitHub or contact rochdi.ferjaoui@polytechnicien.tn
