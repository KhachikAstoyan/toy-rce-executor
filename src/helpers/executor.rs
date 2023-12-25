use super::language::Language;
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::remove_file;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

async fn write_file(file_path: &PathBuf, content: String) -> Result<File, io::Error> {
    let mut file = File::create(file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(file)
}

#[derive(Serialize)]
pub struct ExecutionResult {
    stdout: String,
    stderr: String,
}

impl ExecutionResult {
    pub fn default() -> Self {
        ExecutionResult {
            stdout: String::from(""),
            stderr: String::from(""),
        }
    }
}

pub async fn execute_code(lang: Language, code: String) -> Result<ExecutionResult, io::Error> {
    let id = uuid::Uuid::new_v4();

    let mut host_file_path = std::env::temp_dir();
    host_file_path.push(format!("{}", id));
    host_file_path.set_extension(lang.get_extension());

    let container_file_path =
        Path::new(lang.get_file_name().as_str()).with_extension(lang.get_extension());

    println!(
        "{}, {}",
        host_file_path.to_str().unwrap(),
        container_file_path.to_str().unwrap()
    );

    write_file(&host_file_path, code).await?;

    let cmd = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--network")
        .arg("none")
        .arg("-v")
        .arg(format!(
            "{}:/{}:ro",
            host_file_path.to_str().unwrap(),
            container_file_path.to_str().unwrap()
        ))
        .arg(format!("toyrce:{}", lang.to_string().to_lowercase()))
        .output()
        .await?;

    let stdout = String::from_utf8(cmd.stdout).unwrap();
    let stderr = String::from_utf8(cmd.stderr).unwrap();

    remove_file(host_file_path).await?;

    Ok(ExecutionResult { stdout, stderr })
}
