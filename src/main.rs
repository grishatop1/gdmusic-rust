use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use threadpool::ThreadPool;
use colored::Colorize;
use scraper::Html;
use scraper;
use ureq;
use dirs;


fn main() {
    let path_o = find_gd_folder();
    let mut path = String::new();
    if path_o == None {
        println!("Paste {} folder path:", "GeometryDash".yellow());
        let mut path = String::new();
        std::io::stdin().read_line(&mut path).unwrap();
        path = path.trim().to_string();
        if !Path::new(&path).exists() {
            println!("{}", "Can't find the folder".red());
            return;
        }
    } else {
        path = path_o.unwrap();
    }

    let music_files = get_songs_paths(&path);

    println!("{} found. ({} songs) Press Enter to start!", path.green(), music_files.len());
    std::io::stdin().read(&mut [0]).unwrap();
    
    fs::create_dir_all("./output").unwrap();

    let pool = ThreadPool::new(12);

    for fpath in music_files {
        pool.execute(move || {
            let fname = fpath.file_stem().unwrap().to_str().unwrap();
            let stdout = io::stdout();
            
            let help_str = format!("Working on {} song ID.", fname);
            writeln!(&mut stdout.lock(), "{}", help_str.yellow()).unwrap();
            
            let url = format!("https://www.newgrounds.com/audio/listen/{}", fname);
            let req = ureq::get(&url).call();//
            if let Err(e) = &req {
                let help_str = format!("{} - failed. [{}]", fname, e);
                writeln!(&mut stdout.lock(), "{}", help_str.red()).unwrap();
                return;
            }

            let document = scraper::Html::parse_document(&req.unwrap().into_string().unwrap());
            
            let song_title = get_ng_title(&document);
            let _song_author = get_ng_author(&document);

            let to_copy = format!("./output/{}.mp3", song_title);

            if Path::new(&to_copy).exists() {
                let help_str = format!("Skipping {}, already there.", fname);
                writeln!(&mut stdout.lock(), "{}", help_str.truecolor(190,190,190)).unwrap();
                return;
            }

            let copy_res = fs::copy(fpath.as_os_str(), &to_copy);
            if let Err(c) = &copy_res {
                let help_str = format!("{} - failed to copy the song [{}]", fname, c);
                writeln!(&mut stdout.lock(), "{}", help_str.red()).unwrap();
                return
            }

            let help_str = format!("{} - completed! ({})", song_title, fname);
            writeln!(&mut stdout.lock(), "{}", help_str.green()).unwrap();
        });
        //break; //DEBUG
    }

    pool.join();
    println!("Done!");
}

fn find_gd_folder() -> Option<String> {
    let os = std::env::consts::OS;
    let mut paths: Vec<PathBuf> = Vec::new();
    paths.push(dirs::data_local_dir().unwrap().join("GeometryDash"));
    paths.push(dirs::data_local_dir().unwrap().join("Geometry Dash"));
    if os == "linux" {
        paths.push(dirs::home_dir().unwrap().join(format!(".wine/drive_c/users/{}/AppData/Local/GeometryDash", std::env::var("USER").unwrap())));
        paths.push(dirs::home_dir().unwrap().join(format!(".wine/drive_c/users/{}/AppData/Local/Geometry Dash", std::env::var("USER").unwrap())));
    }

    for p in paths {
        if p.exists() {
            return Some(p.to_str().unwrap().to_string());
        }
    }
    None
}

fn get_ng_title(document: &Html) -> String {
    let title_selector = scraper::Selector::parse(".rated-e").unwrap();
    document.select(&title_selector).next().unwrap().inner_html()
}

fn get_ng_author(document: &Html) -> String {
    let author_selector = scraper::Selector::parse(".item-details-main > h4:nth-child(1) > a:nth-child(1)").unwrap();
    document.select(&author_selector).next().unwrap().inner_html()
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
