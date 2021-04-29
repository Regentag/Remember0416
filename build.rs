extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/ribbon.ico");
    res.set_manifest_file("app.manifest");
    res.compile().unwrap();
  }
}
