extern crate hyper;

use std::os;
use std::io::fs::File;
use hyper::Client;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let out_file = Path::new(os::getenv("OUT_DIR").unwrap()).join("glslang_validator");

    let url = if target.contains("windows") {
        "https://cvs.khronos.org/svn/repos/ogl/trunk/ecosystem/public/sdk/tools/glslang/Install/Windows/glslangValidator.exe"
    } else if target.contains("linux") {
        "https://cvs.khronos.org/svn/repos/ogl/trunk/ecosystem/public/sdk/tools/glslang/Install/Linux/glslangValidator"
    } else {
        panic!("The platform `{}` is not supported", target);
    };

    let mut client = Client::new();

    let mut res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);

    File::create(&out_file).unwrap().write(res.read_to_end().unwrap().as_slice()).unwrap();
}
