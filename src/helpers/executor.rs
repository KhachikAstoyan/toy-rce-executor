use super::language::Language;
use serde::Serialize;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use tokio::fs::remove_file;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

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
    host_file_path.push(id.to_string());
    host_file_path.set_extension(lang.get_extension());

    let container_file_path =
        Path::new(lang.get_file_name().as_str()).with_extension(lang.get_extension());

    write_file(&host_file_path, code).await?;

    let output = run_docker_container(&host_file_path, &container_file_path, &lang).await?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    remove_file(host_file_path).await?;

    Ok(ExecutionResult { stdout, stderr })
}

async fn run_docker_container(
    host_path: &PathBuf,
    container_path: &PathBuf,
    lang: &Language,
) -> Result<Output, io::Error> {
    let cmd = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--network")
        .arg("none")
        .arg("-v")
        .arg(format!(
            "{}:/{}:ro",
            host_path.to_str().unwrap(),
            container_path.to_str().unwrap()
        ))
        .arg(format!("toyrce:{}", lang.to_string().to_lowercase()))
        .output();

    timeout(Duration::from_secs(20), cmd).await?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_code() {
        let code = String::from("console.log('Hello world')");
        let result = execute_code(Language::Javascript, code).await.unwrap();
        assert_eq!(result.stdout, "Hello world\n");
        assert_eq!(result.stderr, "");
    }

    #[tokio::test]
    async fn test_execute_code_with_error() {
        let code = String::from("console.log('Hello world')");
        let result = execute_code(Language::Rust, code).await.unwrap();
        assert_eq!(result.stdout, "");
    }
}
