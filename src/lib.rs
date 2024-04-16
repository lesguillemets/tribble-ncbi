static BASE_URL: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

pub fn f() {
    println!("Hello, world!");
}

pub struct Search {
    pub database: String,
    pub query: String,
}

impl ToURL for Search {
    fn to_url(&self) -> String {
        format!(
            "{BASE_URL}/esearch.fcgi?db={}&term={}",
            self.database, self.query
        )
    }
}

pub trait ToURL {
    fn to_url(&self) -> String;
}
