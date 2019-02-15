#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;
use reqwest::StatusCode;

fn main() {
    let matches = clap_app!(lyrics =>
        (version: "0.1.0")
        (author: "Tyler H. Sowers <thsowers@gmail.com>")
        (about: "Lookup lyrics")
        (@arg ARTIST: +required "Artist to search for")
        (@arg TRACK: +required "Track to search for")
    ).get_matches();

    // Get args
    let artist = str::replace(matches.value_of("ARTIST").unwrap(), " ", "_");
    let song = str::replace(matches.value_of("TRACK").unwrap(), " ", "_");

    get_data(&artist, &song);
}

fn get_data(artist: &str, song: &str) {
    // Fetch Data
    let url = &format!("{}{}:{}", "http://lyrics.wikia.com/wiki/", artist, song);
    let res = reqwest::get(url).unwrap();

    if res.status() == StatusCode::NOT_FOUND {
        println!("Lyrics not found");
        return; // TODO: Fallback to alternative services
    }

    // Parse HTML and pretty print results
    Document::from_read(res).unwrap()
        .find(Class("lyricbox"))
        .for_each(|x| pretty_print(x.inner_html()));
}

fn pretty_print(lyrics: String) {

    // Add linebreaks
    let mut res = str::replace(&lyrics, "<div class=\"lyricsbreak\"></div>", "\n");
    res = str::replace(&res, "<br>", "\n");

    // Split string by newlines
    let lines = res.split("\n");

    // Spacer
    println!("{}", "");
    for line in lines {
        // Print with some indentation
        println!("{:indent$}{}", "", line, indent=4);
    }
}