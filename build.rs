use std::env;

const CONFIG_FILE: &str = "asigntseonline.conf";

fn main() -> std::io::Result<()> {
    let target = env::var("TARGET").expect("TARGET was not set");

    if cfg!(target_os = "windows") {
        let arch = if target.contains("x86_64-pc-windows-msvc") {
            "64bit "
        } else if target.contains("i686-pc-windows-msvc") {
            "32bit "
        } else {
            ""
        };

        let desc = format!("fiskaltrust a.sign TSE wrapper {}", arch);

        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", &desc);
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));

        res.compile().unwrap();
    }
    if Ok("release".to_owned()) == env::var("PROFILE") {
        std::fs::write(format!("./target/release/{}", CONFIG_FILE), std::fs::read_to_string(format!("./src/config/{}", CONFIG_FILE))?)?;
    }

    Ok(())
}
