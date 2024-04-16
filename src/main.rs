use reqwest;
use talk_ncbi::ToURL;
fn main() {
    talk_ncbi::f();
    let db = String::from("pubmed");
    let q = String::from("science[journal]+AND+breast+cancer+AND+2008[pdat]");
    let s = talk_ncbi::Search {
        database: db,
        query: q,
    };
    println!("{}", g(&s).unwrap());
}

fn g(s: &talk_ncbi::Search) -> Option<String> {
    println!("looking at -> {}", s.to_url());
    let body = reqwest::blocking::get(s.to_url()).ok()?.text().ok()?;
    return Some(body);
}
