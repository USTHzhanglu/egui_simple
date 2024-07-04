extern crate embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    embed_resource::compile("windows_style/mainfest.rc", embed_resource::NONE);
}