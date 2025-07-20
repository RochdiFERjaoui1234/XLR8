use std::thread;
use std::time::Duration;
use tauri::Manager;
use winapi::um::processthreadsapi::{OpenProcess, SuspendThread, ResumeThread};
use winapi::um::psapi::{EnumProcesses, GetProcessMemoryInfo, GetModuleFileNameExA, PROCESS_MEMORY_COUNTERS};
use winapi::um::winnt::{PROCESS_ALL_ACCESS, HANDLE, THREAD_SUSPEND_RESUME};
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32};
use winapi::shared::minwindef::{DWORD, BOOL};
use serde::{Serialize, Deserialize};
use std::ptr;
use std::ffi::CString;

mod ml_optimizer;
mod security;
mod virtualizer;

#[derive(Serialize, Deserialize, Clone)]
struct ProcessData {
    pid: u32,
    name: String,
    memory_usage: u64, // in bytes
    status: String,    // "running", "frozen", "suspicious"
}

#[tauri::command]
fn get_process_list() -> Vec<ProcessData> {
    let mut pids: [DWORD; 1024] = [0; 1024];
    let mut cb_needed: DWORD = 0;

    unsafe {
        if EnumProcesses(pids.as_mut_ptr(), std::mem::size_of_val(&pids) as DWORD, &mut cb_needed) == 0 {
            return vec![];
        }

        let num_processes = cb_needed / std::mem::size_of::<DWORD>() as DWORD;
        let mut processes = Vec::new();

        for &pid in &pids[..num_processes as usize] {
            let process_handle: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
            if !process_handle.is_null() {
                let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
                if GetProcessMemoryInfo(process_handle, &mut pmc, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD) != 0 {
                    let mut name_buf = [0u8; 260];
                    let name_len = GetModuleFileNameExA(process_handle, ptr::null_mut(), name_buf.as_mut_ptr() as *mut i8, 260);
                    let name = if name_len > 0 {
                        let name_str = CString::from_raw(name_buf.as_mut_ptr() as *mut i8);
                        name_str.to_string_lossy().into_owned()
                    } else {
                        format!("Process{}", pid)
                    };
                    processes.push(ProcessData {
                        pid,
                        name,
                        memory_usage: pmc.WorkingSetSize as u64,
                        status: "running".to_string(),
                    });
                }
                CloseHandle(process_handle);
            }
        }
        processes
    }
}

#[tauri::command]
fn freeze_process(pid: u32) -> bool {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return false;
        }

        let mut thread_entry: THREADENTRY32 = std::mem::zeroed();
        thread_entry.dwSize = std::mem::size_of::<THREADENTRY32>() as DWORD;

        if Thread32First(snapshot, &mut thread_entry) != 0 {
            loop {
                if thread_entry.th32OwnerProcessID == pid {
                    let thread_handle = OpenThread(THREAD_SUSPEND_RESUME, 0, thread_entry.th32ThreadID);
                    if !thread_handle.is_null() {
                        SuspendThread(thread_handle);
                        CloseHandle(thread_handle);
                    }
                }
                if Thread32Next(snapshot, &mut thread_entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
        true
    }
}

#[tauri::command]
fn resume_process(pid: u32) -> bool {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return false;
        }

        let mut thread_entry: THREADENTRY32 = std::mem::zeroed();
        thread_entry.dwSize = std::mem::size_of::<THREADENTRY32>() as DWORD;

        if Thread32First(snapshot, &mut thread_entry) != 0 {
            loop {
                if thread_entry.th32OwnerProcessID == pid {
                    let thread_handle = OpenThread(THREAD_SUSPEND_RESUME, 0, thread_entry.th32ThreadID);
                    if !thread_handle.is_null() {
                        ResumeThread(thread_handle);
                        CloseHandle(thread_handle);
                    }
                }
                if Thread32Next(snapshot, &mut thread_entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
        true
    }
}

#[tauri::command]
fn enable_turbo_mode() {
    let processes = get_process_list();
    for process in processes {
        if process.memory_usage > 50_000_000 || process.status == "suspicious" {
            freeze_process(process.pid);
        }
    }
}

fn monitor_and_optimize(app_handle: tauri::AppHandle) {
    loop {
        let processes = get_process_list();
        for process in &processes {
            if process.memory_usage > 100_000_000 && process.status != "frozen" {
                if freeze_process(process.pid) {
                    println!("Froze PID: {}", process.pid);
                }
            }
        }
        app_handle.emit_all("update_processes", processes).unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_process_list, freeze_process, resume_process, enable_turbo_mode])
        .setup(|app| {
            let app_handle = app.handle();
            thread::spawn({
                let app_handle = app_handle.clone();
                move || monitor_and_optimize(app_handle)
            });
            thread::spawn({
                let app_handle = app_handle.clone();
                move || ml_optimizer::start_monitoring(app_handle)
            });
            thread::spawn({
                let app_handle = app_handle.clone();
                move || security::start_scanning(app_handle)
            });
            thread::spawn({
                let app_handle = app_handle.clone();
                move || {
                    loop {
                        virtualizer::check_docker(app_handle.clone());
                        thread::sleep(Duration::from_secs(60));
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error running XLR8");
}