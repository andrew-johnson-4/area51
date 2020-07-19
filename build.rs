
fn visit_dirs(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("cargo:warning=file:{:?}", path);
            }
        }
    }
    Ok(())
}

fn main() {
    println!("cargo:warning=in build script");
    let current_dir = std::env::current_dir().expect("current_dir").join("src");
    visit_dirs(&current_dir).expect("visit_dirs");
}
