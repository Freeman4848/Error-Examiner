fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon("assets/app-icon.ico");
    resource.set("ProductName", "Error Examiner");
    resource.set("FileDescription", "Error Examiner");
    resource.set("InternalName", "Error Examiner");
    resource.set("OriginalFilename", "Error-Examiner.exe");
    resource
        .compile()
        .expect("Windows resources should compile");
}
