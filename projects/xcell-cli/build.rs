use std::io;

fn main() -> io::Result<()> {
    if cfg!(target_env = "msvc") {
        winres::WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/icon.ico")
            .compile()?;
    }
    Ok(())
}
