use crate::constants::TEMP_DIR;

use fastuuid::Generator;

#[inline(always)]
pub fn get_random_filename(ext: Option<&str>) -> String {
    let uuid = Generator::new()
        .hex128_as_string()
        .expect("Generating UUID should work");
    if let Some(ext) = ext {
        format!("{}.{}", uuid, ext)
    } else {
        uuid
    }
}

#[inline]
pub fn get_temp_file_path() -> String {
    let uuid = get_random_filename(None);
    let file_path = format!("/{}/{}", TEMP_DIR, uuid);

    // create the temp directory if it doesn't exist
    std::fs::create_dir_all(TEMP_DIR).unwrap();
    file_path
}
