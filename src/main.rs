use colored::Colorize;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use ureq;
use threadpool::ThreadPool;

fn main() {
    println!("Paste {} folder path: [leave empty for default]", "GeometryDash".yellow());
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
    let pool = ThreadPool::new(4);

    for fpath in music_files {
        pool.execute(move || {
            let fname = fpath.file_name().unwrap().to_str().unwrap();
            println!("{}", fname);

            //let stdout = io::stdout();
            //writeln!(&mut stdout.lock(), "lol").unwrap();
        });
    }

    pool.join();
    println!("{}", "All jobs are done!".green());
}

fn get_songs_paths(path: &str) -> Vec<PathBuf> {
    let all_files = fs::read_dir(&path).unwrap();
    let mut music_files: Vec<PathBuf> = Vec::new();

    for res_path in all_files {
        if res_path.as_ref().unwrap().path().extension().unwrap().to_str().unwrap() == "mp3" {
            music_files.push(res_path.unwrap().path());
        }
    }

    music_files
}
