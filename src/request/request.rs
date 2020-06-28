use crate::request::url::{Endpoint, PageParse, PrefixSearch};
use regex::Regex;
use reqwest::blocking::Client;

pub struct Request {
    client: Client,
    url: String,
    endpoint: Endpoint,
    pub page_list: Vec<String>,
    pub toc_list: Vec<String>,
    pub page: Vec<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: String::new(),
            endpoint: Endpoint::PrefixSearch,
            page_list: Vec::new(),
            toc_list: Vec::new(),
            page: Vec::new(),
        }
    }
}

impl<'a> Request {
    pub fn new(search: &'a str, url: Endpoint) -> Self {
        let mut req = Self::default();
        match url {
            Endpoint::PrefixSearch => {
                req.endpoint = Endpoint::PrefixSearch;
                req.url = PrefixSearch::new()
                    .search(search)
                    .limit(30)
                    .offset(0)
                    .parse();
            }
            Endpoint::PageParse => {
                req.endpoint = Endpoint::PageParse;
                req.url = PageParse::new().page(search).parse();
            }
        }
        req
    }

    pub fn fetch(mut self) -> Self {
        let res = self.client.get(&self.url).send().unwrap().text().unwrap();
        match self.endpoint {
            Endpoint::PrefixSearch => {
                let re = Regex::new(r#""title":"(.+?)""#).unwrap(); //TODO: lazy_static!
                for each in re.captures_iter(&res) {
                    if let Some(match_) = each.get(1) {
                        self.page_list.push(match_.as_str().to_owned());
                    }
                }
            }
            Endpoint::PageParse => {
                let re = Regex::new(r#""sections":(.+),"wikitext":(.+)"#).unwrap(); //TODO: lazy_static!
                for each in re.captures_iter(&res) {
                    // Parse sections (toc)
                    if let Some(match_) = each.get(1) {
                        let re = Regex::new(r#""line":"(.+?)".*?"number":"(.+?)""#).unwrap(); //TODO: lazy_static!
                        for each in re.captures_iter(&match_.as_str()) {
                            if let Some(match_1) = each.get(1) {
                                if let Some(match_2) = each.get(2) {
                                    let txt = match_1.as_str();
                                    let num = match_2.as_str();
                                    let indent = "  ".repeat(num.matches(".").count());
                                    self.toc_list.push(format!("{} {} {}", indent, num, txt));
                                }
                            }
                        }
                    }
                    // Parse wikitext
                    if let Some(match_) = each.get(2) {
                        let text = match_.as_str();
                        let re = Regex::new(
                            r#"\{\{.+?\}\}|<ref\\s*>.+?</ref>|<ref .+?>|\[\[File.+?\]\]|<.ref>"#,
                        )
                        .unwrap(); //TODO: lazy_static!
                        let text = re.replace_all(text, "");
                        let re = Regex::new(r#"\\n\\n\\n"#).unwrap(); //TODO: lazy_static!
                        let text = re.replace_all(&text, "");
                        let re = Regex::new(r#"\\n\\n\\n"#).unwrap(); //TODO: lazy_static!
                        let text = re.replace_all(&text, "");
                        for each in text.split("\\n") {
                            if each == "\"" {
                                continue;
                            }
                            self.page.push(format!("{}\n", each));
                        }
                    }
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_fetch_prefix_search() {
        let res = Request::new("rust", Endpoint::PrefixSearch).fetch();
        assert_eq!(res.page_list, vec!["Rust".to_owned()]);
    }

    #[test]
    fn request_fetch_parse_page() {
        let res = Request::new("rust", Endpoint::PageParse).fetch();
        // assert_eq!(res.page_list, vec!["Rust".to_owned()]);
    }
}
