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
		let wiki = answer.json::<WikiSearchTmp>()?;
		Ok(wiki.parse)
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
