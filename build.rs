fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/app-icon.ico");
        resource.set("ProductName", "Error Examiner");
        resource.set("FileDescription", "Local AI error and incident examiner");
        resource.set("OriginalFilename", "Error-Examiner.exe");
        resource
            .compile()
            .expect("Windows resources should compile");
    }
}
