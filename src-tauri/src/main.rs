#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::{Path, PathBuf};

/// 任务目录：优先 exe 同级的 tasks/；不可写时退回「文档\T-Minus\tasks」
fn tasks_dir() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let dir = exe.parent().unwrap_or(Path::new(".")).to_path_buf();
    if is_writable(&dir) {
        return dir.join("tasks");
    }
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    PathBuf::from(home)
        .join("Documents")
        .join("T-Minus")
        .join("tasks")
}

/// 探测目录是否可写（Program Files 之类普通权限写不进去）
fn is_writable(dir: &Path) -> bool {
    let probe = dir.join(".tminus-write-test");
    match fs::write(&probe, b"x") {
        Ok(_) => {
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

/// 确保目录存在（被删掉就重建）
fn ensure_dir() -> Result<PathBuf, String> {
    let d = tasks_dir();
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    Ok(d)
}

#[tauri::command]
fn tasks_path() -> String {
    ensure_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn list_tasks() -> Result<Vec<String>, String> {
    let d = ensure_dir()?;
    let mut out: Vec<String> = Vec::new();
    for entry in fs::read_dir(&d).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let p = entry.path();
        let is_json = p
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("json"))
            .unwrap_or(false);
        if is_json {
            if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                out.push(stem.to_string());
            }
        }
    }
    out.sort();
    Ok(out)
}

#[tauri::command]
fn read_task(name: String) -> Result<String, String> {
    let d = ensure_dir()?;
    fs::read_to_string(d.join(format!("{}.json", name))).map_err(|e| e.to_string())
}

/// 原子写入：先写 .tmp，再替换目标文件
#[tauri::command]
fn write_task(name: String, content: String) -> Result<(), String> {
    let d = ensure_dir()?;
    let target = d.join(format!("{}.json", name));
    let tmp = d.join(format!("{}.json.tmp", name));
    fs::write(&tmp, content.as_bytes()).map_err(|e| e.to_string())?;
    if target.exists() {
        fs::remove_file(&target).map_err(|e| e.to_string())?;
    }
    fs::rename(&tmp, &target).map_err(|e| e.to_string())
}

#[tauri::command]
fn rename_task(from: String, to: String) -> Result<(), String> {
    let d = ensure_dir()?;
    let src = d.join(format!("{}.json", from));
    let dst = d.join(format!("{}.json", to));
    if dst.exists() {
        return Err("exists".into());
    }
    fs::rename(src, dst).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_task(name: String) -> Result<(), String> {
    let d = ensure_dir()?;
    let p = d.join(format!("{}.json", name));
    if !p.exists() {
        return Ok(());
    }
    trash::delete(&p).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_fullscreen(window: tauri::Window, on: bool) -> Result<(), String> {
    window.set_fullscreen(on).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_tasks_dir() -> Result<(), String> {
    let d = ensure_dir()?;
    std::process::Command::new("explorer")
        .arg(d.as_os_str())
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // 启动即确保 tasks 目录存在（被删掉就重建）
            let _ = ensure_dir();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tasks_path,
            list_tasks,
            read_task,
            write_task,
            rename_task,
            delete_task,
            set_fullscreen,
            open_tasks_dir
        ])
        .run(tauri::generate_context!())
        .expect("failed to run T-Minus");
}
