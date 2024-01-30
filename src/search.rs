// https://en.wikipedia.org/w/api.php?action=parse&page=chese&redirects&prop=langlinks&format=json

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct WikiSearchTmp {
	parse: WikiSearch,
}

#[derive(Debug, Deserialize)]
pub struct WikiEntry {
	lang: String,
	#[serde(rename = "*")]
	name: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiSearch {
	title: String,
	langlinks: Vec<WikiEntry>,
}

// const URL: &str = "https://en.wikipedia.org/w/api.php";
fn url(lang: &str) -> String {
	format!("https://{}.wikipedia.org/w/api.php", lang)
}

#[derive(Debug, Serialize)]
struct Query<'a> {
	action: &'static str,
	page: &'a str,
	redirects: bool,
	prop: &'static str,
	format: &'static str,
}

impl<'a> Query<'a> {
	fn new(page: &str) -> Query {
		Query {
			action: "parse",
			page,
			redirects: true,
			prop: "langlinks",
			format: "json",
		}
	}
}

impl WikiSearch {
	pub fn search() -> Self {
		let reqwest = reqwest::blocking::Client::new();
		let query = Query::new("chese");

		let url = url("en");
		let answer = reqwest.get(&url).query(&query);
		println!("rq {:?}", answer);

		let answer = reqwest.get(url).query(&query).send().unwrap();
		let req = answer.json::<WikiSearchTmp>().unwrap();
		req.parse
	}

	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn langs(&self) -> &[WikiEntry] {
		&self.langlinks
	}
}
