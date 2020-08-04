use shaderc::CompilationArtifact;

pub fn load_compiled_vert() -> CompilationArtifact {
    let vs_src = include_str!("shader.vert");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();

    vs_spirv
}

pub fn load_compiled_frag() -> CompilationArtifact {
    let fs_src = include_str!("shader.frag");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();

    fs_spirv
}
