use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    let wgsl = std::fs::read_to_string("in.wgsl")?;

    let glsl_filename = wgsl
        .lines()
        .next()
        .map(|line| line.strip_prefix("//include:"))
        .flatten();

    let glsl_module = if let Some(glsl_filename) = glsl_filename {
        let glsl = std::fs::read_to_string(glsl_filename)?;
        let mut glsl_parser = naga::front::glsl::Parser::default();
        glsl_parser
            .parse(
                &naga::front::glsl::Options {
                    stage: None,
                    defines: naga::FastHashMap::default(),
                },
                &glsl,
            )
            .expect("glsl parse")
    } else {
        naga::Module::default()
    };

    let mut wgsl_parser = naga::front::wgsl::Parser::new();
    let wgsl_module = wgsl_parser
        .parse_to_module(&wgsl, glsl_module)
        .context("wgsl parse")?;

    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        naga::valid::Capabilities::default(),
    );

    let module_info = validator.validate(&wgsl_module)?;
    let out = naga::back::wgsl::write_string(&wgsl_module, &module_info)?;

    println!("{}", out);

    Ok(())
}
