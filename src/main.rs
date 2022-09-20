use colored::Colorize;
use std::path::Path;
use std::fs;

fn main() {
    println!("Paste {} folder path: [leave emptry for default]", "GeometryDash".green());
    let mut path = String::new();
    std::io::stdin().read_line(&mut path).unwrap();
    path = path.trim().to_string();
    if path.is_empty() {
        path = "/home/grisha/.wine/drive_c/users/grisha/AppData/Local/GeometryDash".into();
    }
    if !Path::new(&path).exists() {
        panic!("Can't find the folder!");
    }

    let music_files = get_songs_paths(&path);
}

fn get_songs_paths(path: &str) -> Vec<String> {
    let all_files = fs::read_dir(&path).unwrap();
    let mut music_files: Vec<String> = Vec::new();

    for res_path in all_files {
        let fpath = res_path.unwrap().path().to_str().unwrap().to_string();
        if !fpath.ends_with(".mp3") {
            continue;
        }
        music_files.push(fpath);
    }

    music_files
}
