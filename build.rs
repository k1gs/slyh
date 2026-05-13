use std::io;

fn main() -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile()?;
    }
    Ok(())
}