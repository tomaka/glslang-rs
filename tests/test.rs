extern crate glslang;

#[test]
fn compilation_success() {
    let r = glslang::test_shaders(vec![("
        #version 110

        uniform mat4 matrix;

        attribute vec3 position;
        attribute vec3 normal;
        varying vec3 v_position;
        varying vec3 v_normal;

        void main() {
            v_position = position;
            v_normal = normal;
            gl_Position = vec4(v_position, 1.0) * matrix;
        }
    ", glslang::ShaderType::Vertex),

    // fragment shader
    ("
        #version 110

        varying vec3 v_normal;

        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        void main() {
            float lum = max(dot(v_normal, normalize(LIGHT)), 0.0);
            vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
            gl_FragColor = vec4(color, 1.0);
        }
    ", glslang::ShaderType::Fragment)]);

    match r {
        glslang::TestResult::Ok => (),
        _ => panic!()
    }
}

#[test]
fn compilation_error() {
    let r = glslang::test_shaders(vec![("
        hello world
    ", glslang::ShaderType::Vertex)]);

    match r {
        glslang::TestResult::Error(_) => (),
        r => panic!("{:?}", r)
    }
}

#[test]
#[ignore]   // fails
fn linking_error() {
    let r = glslang::test_shaders(vec![("
        #version 110

        uniform mat4 matrix;

        attribute vec3 position;
        attribute vec3 normal;
        varying vec3 v_position;
        varying vec3 v_normal;

        void main() {
            v_position = position;
            v_normal = normal;
            gl_Position = vec4(v_position, 1.0) * matrix;
        }
    ", glslang::ShaderType::Vertex),

    // fragment shader
    ("
        #version 110

        varying vec4 v_normal;

        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        void main() {
            float lum = max(dot(v_normal.xyz, normalize(LIGHT)), 0.0);
            vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
            gl_FragColor = vec4(color, 1.0);
        }
    ", glslang::ShaderType::Fragment)]);

    match r {
        glslang::TestResult::Error(_) => (),
        r => panic!("{:?}", r)
    }
}
