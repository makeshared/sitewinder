use lazy_static::lazy_static;
use regex::Regex;

macro_rules! def_regex {
    ($name:ident, $pattern:expr) => {
        lazy_static! {
            pub static ref $name: Regex = Regex::new($pattern)
                .expect(concat!("Failed to compile ", stringify!($name), " regex"));
        }
    };
}

def_regex!(RE_INCLUDE, r#"\{\s*include\s+\"(?<path>[^\"]+)\"\s*\}"#);
def_regex!(RE_TITLE, r#"\{\s*title\s*\}"#);
def_regex!(RE_DATE, r#"\{\s*date\s*\}"#);
def_regex!(RE_AUTHOR, r#"\{\s*author\s*\}"#);
def_regex!(RE_CURRENT_YEAR, r#"\{\s*current_year\s*\}"#);
def_regex!(RE_GROUP_NAV, r#"\{\s*group\s+\"(?<path>[^\"]+)\"\s*\}"#);
def_regex!(RE_PREV_TITLE, r#"\{\s*prev.title\s*\}"#);
def_regex!(RE_PREV_PATH, r#"\{\s*prev.path\s*\}"#);
def_regex!(RE_NEXT_TITLE, r#"\{\s*next.title\s*\}"#);
def_regex!(RE_NEXT_PATH, r#"\{\s*next.path\s*\}"#);
def_regex!(RE_TAGS, r#"\{\s*tags\s+'(?<markup>.*)'\s*\}"#);
def_regex!(RE_TAG_PAGE_LINK, r#"\{\s*tag.page.link\s*\}"#);
def_regex!(RE_TAG_PAGE_LINK_SIZE, r#"\{\s*tag.page.link_size\s*\}"#);
def_regex!(RE_TAG_PAGE_TITLE, r#"\{\s*tag.page.title\s*\}"#);
def_regex!(RE_PAGES, r#"\{\s*pages\s+'(?<link>.*)'\s*\}"#);
def_regex!(RE_PAGE_LINK, r#"\{\s*page.link\s*\}"#);
def_regex!(RE_PAGE_TITLE, r#"\{\s*page.title\s*\}"#);
def_regex!(RE_LINK_HREF, r#"(<(?:a|link|area|base)\s+[^>]*href\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
def_regex!(RE_LINK_SRC, r#"(<(?:img|audio|video|script|source|iframe|embed|track)\s+[^>]*src\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
def_regex!(RE_LINK_DATA, r#"(<(?:object|embed)\s+[^>]*data\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
def_regex!(RE_LINK_POSTER, r#"(<(?:video)\s+[^>]*poster\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
def_regex!(RE_LINK_ACTION, r#"(<form\s+[^>]*action\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
def_regex!(RE_LINK_SRCSET, r#"(<(?:img|source)\s+[^>]*srcset\s*=\s*["'])([^"']+)(["'][^>]*>)"#);
