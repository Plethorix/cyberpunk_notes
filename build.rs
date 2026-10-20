fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        if std::path::Path::new("icon.ico").exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("icon.ico");
            res.compile().unwrap();
        }
    }
}
