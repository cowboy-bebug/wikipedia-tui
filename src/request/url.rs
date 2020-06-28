const BASE_URL: &str = "https://en.wikipedia.org/w/api.php?format=json";
const PREFIX_SEARCH: &str = "action=query&list=prefixsearch";
const PAGE_PARSE: &str = "action=parse&prop=wikitext%7Csections&formatversion=2";

pub enum Endpoint {
    PrefixSearch,
    PageParse,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PrefixSearch<'a> {
    url: &'a str,
    params: &'a str,
    search: &'a str,
    limit: u16,
    offset: u16,
}

impl Default for PrefixSearch<'_> {
    fn default() -> Self {
        Self {
            url: BASE_URL,
            params: PREFIX_SEARCH,
            search: <&str as std::default::Default>::default(),
            limit: u16::default(),
            offset: u16::default(),
        }
    }
}

impl<'a> PrefixSearch<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(&mut self, search: &'a str) -> &mut Self {
        self.search = search;
        self
    }

    pub fn limit(&mut self, limit: u16) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn offset(&mut self, offset: u16) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn parse(&self) -> String {
        format!(
            "{}&{}&pssearch={}&pslimit={}&psoffset={}",
            self.url, self.params, self.search, self.limit, self.offset
        )
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PageParse<'a> {
    url: &'a str,
    params: &'a str,
    page: &'a str,
}

impl Default for PageParse<'_> {
    fn default() -> Self {
        Self {
            url: BASE_URL,
            params: PAGE_PARSE,
            page: <&str as std::default::Default>::default(),
        }
    }
}

impl<'a> PageParse<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(&mut self, page: &'a str) -> &mut Self {
        self.page = page;
        self
    }

    pub fn parse(&self) -> String {
        format!("{}&{}&page={}", self.url, self.params, self.page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_search_new() {
        assert_eq!(
            PrefixSearch::new(),
            PrefixSearch {
                url: BASE_URL,
                params: PREFIX_SEARCH,
                search: <&str as std::default::Default>::default(),
                limit: u16::default(),
                offset: u16::default(),
            }
        )
    }

    #[test]
    fn prefix_search_offset() {
        assert_eq!(
            PrefixSearch::new().search("test").limit(7357).offset(7357),
            &PrefixSearch {
                url: BASE_URL,
                params: PREFIX_SEARCH,
                search: "test",
                limit: 7357,
                offset: 7357,
            }
        )
    }

    #[test]
    fn prefix_search_parse() {
        let url = PrefixSearch::new()
            .search("test")
            .limit(7357)
            .offset(7357)
            .parse();
        let expected_url = format!(
            "{}&{}&{}",
            BASE_URL, PREFIX_SEARCH, "pssearch=test&pslimit=7357&psoffset=7357"
        );
        assert_eq!(url, expected_url)
    }

    #[test]
    fn page_parse_new() {
        assert_eq!(
            PageParse::new(),
            PageParse {
                url: BASE_URL,
                params: PAGE_PARSE,
                page: <&str as std::default::Default>::default(),
            }
        )
    }

    #[test]
    fn page_parse_page() {
        assert_eq!(
            PageParse::new().page("test"),
            &PageParse {
                url: BASE_URL,
                params: PAGE_PARSE,
                page: "test",
            }
        )
    }

    #[test]
    fn page_parse_parse() {
        let url = PageParse::new().page("test").parse();
        let expected_url = format!("{}&{}&{}", BASE_URL, PAGE_PARSE, "page=test");
        assert_eq!(url, expected_url)
    }
}
