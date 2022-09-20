use colored::Colorize;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use ureq;
use threadpool::ThreadPool;
use scraper;

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

    fs::create_dir_all("./output").unwrap();

    let music_files = get_songs_paths(&path);
    let pool = ThreadPool::new(4);

    for fpath in music_files {
        pool.execute(move || {
            let fname = fpath.file_stem().unwrap().to_str().unwrap();
            let stdout = io::stdout();
            
            let help_str = format!("Working on {} song ID.", fname);
            writeln!(&mut stdout.lock(), "{}", help_str.yellow()).unwrap();
            
            let url = format!("https://www.newgrounds.com/audio/listen/{}", fname);
            let req = ureq::get(&url).call();//
            if let Err(_req) = &req {
                let help_str = format!("{} - failed.", fname);
                writeln!(&mut stdout.lock(), "{}", help_str.red()).unwrap();
                return;
            }

            let document = scraper::Html::parse_document(&req.unwrap().into_string().unwrap());

            let title_selector = scraper::Selector::parse(".rated-e").unwrap();
            let song_title = document.select(&title_selector).next().unwrap().inner_html();
            
            let author_selector = scraper::Selector::parse(".item-details-main > h4:nth-child(1) > a:nth-child(1)").unwrap();
            let song_author = document.select(&author_selector).next().unwrap().inner_html();


            let help_str = format!("{} - completed!", fname);
            writeln!(&mut stdout.lock(), "{}", help_str.green()).unwrap();
        });
        break; //DEBUG
    }

    pool.join();
    println!("Done!");
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
