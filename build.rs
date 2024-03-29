use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("ahoy.exe.manifest")
            .set_icon("resources/ahoy.ico");
        res.compile()?;
    }

    Ok(())
}
