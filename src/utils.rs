use log::warn;
use std::env;

pub fn get_exe_name() -> String {
    let args: Vec<String> = env::args().collect();

    if let Some(f) = args.first() {
        return std::path::PathBuf::from(f)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
    }
    warn!("Impossible to get executable name");

    String::default()
}
