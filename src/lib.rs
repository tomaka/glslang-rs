#![unstable]

use std::io::TempDir;
use std::io::fs::File;
use std::io::process::Command;

#[derive(Show, Clone, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute,
}

/// Result of a compilation attempt.
#[derive(Show, Clone)]
pub enum TestResult {
    Ok,
    Error(String),
}

/// Tries to compile and link the shaders together.
pub fn test_shaders(shaders: Vec<(&str, ShaderType)>) -> TestResult {
    let temp_dir = TempDir::new("glslang-compile").unwrap();

    let mut command = Command::new(concat!(env!("OUT_DIR"), "/glslang_validator"));
    command.arg("-l");

    for (num, (source, ty)) in shaders.into_iter().enumerate() {
        let extension = match ty {
            ShaderType::Vertex => ".vert",
            ShaderType::Fragment => ".frag",
            ShaderType::Geometry => ".geom",
            ShaderType::TessellationControl => ".tesc",
            ShaderType::TessellationEvaluation => ".tese",
            ShaderType::Compute => ".comp",
        };

        let file_path = temp_dir.path().join(format!("{}{}", num, extension));
        File::create(&file_path).unwrap().write_str(source).unwrap();
        command.arg(file_path);
    }

    let command = command.output().unwrap();

    let result = if command.status.success() {
        TestResult::Ok
    } else {
        TestResult::Error(String::from_utf8(command.output.clone()).unwrap())
    };

    temp_dir.close().unwrap();

    result
}
