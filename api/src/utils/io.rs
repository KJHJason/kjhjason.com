use crate::constants::constants::TEMP_DIR;
use fastuuid::Generator;

pub fn get_temp_file_path() -> String {
    let uuid = Generator::new().hex128_as_string().unwrap();
    let file_path = format!("./{}/{}", TEMP_DIR, uuid);

    // create the temp directory if it doesn't exist
    std::fs::create_dir_all(TEMP_DIR).unwrap();
    return file_path;
}
