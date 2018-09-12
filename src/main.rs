extern crate reqwest;
extern crate select;

use std::env;
use select::document::Document;
use select::predicate::Class;

fn main() {
    let args: Vec<String> = env::args().collect();
    let artist = str::replace(&args[1], " ", "_");
    let song = str::replace(&args[2], " ", "_");

    get_data(&artist, &song);
}

fn get_data(artist: &str, song: &str) {
    let url = &format!("{}{}:{}", "http://lyrics.wikia.com/wiki/", artist, song);
    let res = reqwest::get(url).unwrap();

    Document::from_read(res).unwrap()
        .find(Class("lyricbox"))
        .for_each(|x| pretty_print(x.inner_html()));
}

fn pretty_print(lyrics: String) {
    println!("{}", "");
    let mut res = str::replace(&lyrics, "<div class=\"lyricsbreak\"></div>", "\n");
    res = str::replace(&res, "<br>", "\n");
    print!("{}", res)
}