use std::fmt::Display;

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

impl WikiEntry {
	pub fn name(&self) -> &str {
		&self.name
	}
}

impl PartialEq<str> for WikiEntry {
	fn eq(&self, other: &str) -> bool {
		self.lang == other
	}
}

#[derive(Debug, Deserialize)]
struct WikiErrorTmp {
	error: WikiError,
}

#[derive(Debug, Deserialize)]
struct WikiError {
	info: String,
}

impl Display for WikiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.info)
	}
}

impl std::error::Error for WikiError {}

#[derive(Debug, Deserialize)]
pub struct WikiSearch {
	title: String,
	langlinks: Vec<WikiEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WikiResponse {
	Res(WikiSearchTmp),
	Err(WikiErrorTmp),
}

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
	fn new(word: &str) -> Query {
		Query {
			action: "parse",
			page: word,
			redirects: true,
			prop: "langlinks",
			format: "json",
		}
	}
}

impl WikiSearch {
	pub fn search(word: &str, lang: &str) -> color_eyre::Result<Self> {
		let reqwest = reqwest::blocking::Client::new();
		let query = Query::new(word);

		let url = url(lang);
		let answer = reqwest.get(url).query(&query).send()?;
		let wiki = answer.json::<WikiResponse>()?;

		match wiki {
			WikiResponse::Res(wiki) => Ok(wiki.parse),
			WikiResponse::Err(werr) => Err(From::from(werr.error)),
		}
	}

	pub fn title(&self) -> &str {
		&self.title
	}

	fn langs(&self) -> &[WikiEntry] {
		&self.langlinks
	}

	pub fn find(&self, lang: &str) -> Option<&WikiEntry> {
		let langs = self.langs();
		langs.iter().find(|entry| entry == &lang)
	}
}
