use clap::{App, Arg, SubCommand};
use download_rs::async_download::Download;
use std::fs;
use substring::Substring;
fn main() {
    let app = App::new("Netease songs downloader")
        .version("0.0.1")
        .author("Caviar-X <caviarx@163.com>")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("the url of the music you need to download (must be netease)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .value_name("SAVE")
                .help("Where the music saves"),
        )
        .get_matches();
    let mut url = app.value_of("url").unwrap().to_string();
    let mut save = "".to_string();
    let mut id_flag = true;
    url.clone().chars().map(|a| id_flag = a.is_numeric());
    if id_flag {
        url = format!("http://music.163.com/#/m/song?id={}", url.clone());
    }
    let clone = url.clone();
    if let None = app.value_of("save") {
        save = url
            .clone()
            .substring(url.rfind("=").unwrap() + 1, url.len())
            .to_string()
            + ".mp3";
    } else {
        save = app.value_of("save").unwrap().to_string();
    }
    match Download::new(
        clone
            .replace("/#/m/song?id=", "/song/media/outer/url?id=")
            .as_str(),
        Some(save.as_str()),
        None,
    )
    .download()
    {
        Ok(_) => {
            println!("Download successfully");
        }
        Err(e) => {
            println!("\x1b[31mError: Failed to download with the error {}", e);
        }
    }
}
//http://music.163.com/#/m/song?id=1359210610
