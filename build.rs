use std::os;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let out_file = Path::new(os::getenv("OUT_DIR").unwrap()).join("glslang_validator");

    let path = if target.contains("windows") {
        Path::new("bin/glslangValidator.exe")
    } else if target.contains("linux") {
        Path::new("bin/glslangValidator")
    } else {
        panic!("The platform `{}` is not supported", target);
    };

    if let Err(_) = std::io::fs::link(&path, &out_file) {
        std::io::fs::copy(&path, &out_file).unwrap();
    }

    std::io::fs::chmod(&out_file, std::io::USER_EXEC).unwrap();
}
