use std::fs::File;

pub fn get_asset() -> File {
    std::fs::File::open("music/Decretum.mp3").unwrap()
}
