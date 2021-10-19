use anyhow::Context;

fn do_file(filename: &str, mut module: naga::Module) -> Result<naga::Module, anyhow::Error> {
    let source = std::fs::read_to_string(filename)?;

    let ext = std::path::Path::new(filename)
        .extension()
        .map(std::ffi::OsStr::to_string_lossy)
        .map(std::borrow::Cow::into_owned);

    for line in source.lines() {
        let include_file = line.strip_prefix("//include:");
        if let Some(include_file) = include_file {
            module = do_file(include_file, module).with_context(||format!("Parsing {}", include_file))?;
        }
    }

    if let Some(ext) = ext {
        match ext.as_str() {
            "wgsl" => {
                let mut wgsl_parser = naga::front::wgsl::Parser::new();
                let module = wgsl_parser
                    .parse_to_module(&source, module)
                    .map_err(|err|wgsl_parse_error(err, &source))
                    .context("wgsl parse")?;

                return Ok(module);
            }
            "glsl" => {
                let mut glsl_parser = naga::front::glsl::Parser::default();
                let module = glsl_parser
                    .parse_to_module(
                        &naga::front::glsl::Options {
                            stage: None,
                            defines: naga::FastHashMap::default(),
                        },
                        &source,
                        module,
                    )
                    .map_err(GlslError::from)
                    .context("glsl parse")?;

                return Ok(module);
            }
            _ => {}
        }
    }
    Err(anyhow::anyhow!("Unknown shader extension {ext}"))
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::env::args().nth(1).unwrap_or_else(|| "example.wgsl".into());

    let module = do_file(&input, naga::Module::default()).with_context(||format!("Parsing {}", input))?;

    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        naga::valid::Capabilities::default(),
    );

    let module_info = validator.validate(&module)?;
    let out = naga::back::wgsl::write_string(&module, &module_info)?;

    println!("{}", out);

    Ok(())
}





fn wgsl_parse_error(error: naga::front::wgsl::ParseError, source: &str) -> anyhow::Error {
    anyhow::anyhow!(error.emit_to_string(source))
}

    

// anyhow doesn't quite deal with the Vec<Error> err variant of glsl-in

#[derive(Debug)]
struct GlslError {
    errs: Vec<naga::front::glsl::Error>,
}
impl GlslError {
    fn from(errs: Vec<naga::front::glsl::Error>) -> GlslError {
        GlslError { errs }
    }
}
impl std::fmt::Display for GlslError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "glsl parse error count {}", self.errs.len())?;
        Ok(())
    }
}
impl std::error::Error for GlslError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.errs[0])
    }
}
