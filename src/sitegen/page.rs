use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::{write, File};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use crate::sitegen::metadata::Metadata;
use crate::sitegen::regexes::*;
pub use crate::sitegen::TagPage;

// TemplateSource enum defines the source of the template content,
// either from a file on disk or from a string in memory.
#[derive(Clone, Debug)]
pub enum TemplateSource {
    File(),
    Memory(String),
}

#[derive(Debug)]
pub struct Page {
    // Absolute path to local folder storing the site files
    root_path: PathBuf,

    // Absolute path to generated output/HTML file
    output_path: PathBuf,

    // The current year, used for { current_year } replacements in templates.
    current_year: String,

    // Page title, author, date, etc.
    metadata: Metadata,

    // The contents of the page, which will change as we process the template.
    // This will contain the final HTML content of the page.
    contents: String,
}

impl Page {
    pub fn new(root_path: &std::path::Path, template_path: &PathBuf, template_source: &TemplateSource, current_year: &str) -> Page {
        // The generated HTML document will be stored in the same folder as the template,
        // but with the file extension changed to .html.
        let mut output_path_local = template_path.to_path_buf();
        output_path_local.set_extension("html");

        let mut template_contents = String::new();

        match template_source {
            TemplateSource::File() => {
                // Read the template file from disk
                let mut file = File::open(template_path).unwrap();
                file.read_to_string(&mut template_contents).unwrap();
            },
            TemplateSource::Memory(contents) => {
                // Use the provided string as the template contents
                template_contents = contents.clone();
            }
        }

        Page {
            root_path: root_path.to_path_buf(),
            output_path: output_path_local.clone(),
            metadata: Metadata::new(&Page::get_page_path(root_path, output_path_local.clone())),
            contents: template_contents.clone(),
            current_year: current_year.to_string(),
        }
    }

    fn get_page_path(root_path: &std::path::Path, output_path_local: PathBuf) -> String {
        let output_path_site = output_path_local.clone().strip_prefix(root_path).unwrap().to_path_buf();
        let mut page_path = output_path_site.to_str().unwrap().to_string();
        if !page_path.starts_with('/') {
            page_path.insert(0, '/');
        }
        page_path
    }

    // Process the .sgpage template and extract metadata (title, group, tags, date, author) from it.
    pub fn process_metadata(&mut self) -> Result<()> {
        let mut reader = BufReader::new(self.contents.as_bytes());

        let mut reading_metadata = false;
        let mut line = String::new();
        let mut processed_contents = String::new();

        while reader.read_line(&mut line).is_ok() {
            if line.eq("\n") || line.eq("\r\n") {
                // Ignore empty lines
            } else if line.starts_with("--") {
                reading_metadata = !reading_metadata;
            } else if !reading_metadata {
                processed_contents.push_str(&line);
                break;
            } else if let Some(keyval) = line.split_once(':') {
                let key = keyval.0.trim();
                let val = keyval.1.trim().to_string();
                match key {
                    "title" => self.metadata.title = val,
                    "group" => self.metadata.group = Some(val),
                    "tags" => {
                        self.metadata.tags =
                            Some(val.split(',').map(str::trim).map(str::to_string).collect())
                    }
                    "date" => {
                        self.metadata.date = Some(NaiveDate::parse_from_str(&val, "%Y-%m-%d").unwrap())
                    }
                    "author" => self.metadata.author = val,
                    _ => eprintln!("ignoring unknown key '{}'", key),
                }
            }
            line.clear();
        }

        // Store the remaining contents after metadata processing
        // This will be the actual HTML content of the page.
        reader.read_to_string(&mut processed_contents)
            .context("Unable to read template contents")?;
        self.contents = processed_contents;

        Ok(())
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn generate(&mut self, prev: Option<Metadata>, next: Option<Metadata>, tags: &BTreeMap<String, TagPage>) {
        println!("generating page '{}'", &self.metadata.path);

        // Process { include "<path>" } blocks
        self.process_includes();

        // Process { title }
        self.contents = self.replace_all(&RE_TITLE, &self.metadata.title);

        // Process { date }
        let date_string = self.metadata.date.map_or(
            String::new(), |date| date.format("%Y-%m-%d").to_string());
        self.contents = self.replace_all(&RE_DATE, &date_string);

        // Process { author }
        self.contents = self.replace_all(&RE_AUTHOR, &self.metadata.author);

        // Process { current_year }
        self.contents = self.replace_all(&RE_CURRENT_YEAR, &self.current_year);

        // Process { group "<path>" } conditional includes
        self.contents = RE_GROUP_NAV
            .replace_all(&self.contents, |caps: &regex::Captures| {
                match &self.metadata.group {
                    Some(_) => {
                        // Page is part of a group, include the file at the specified path
                        let path = self.get_local_include_path(caps.name("path").unwrap().as_str());
                        std::fs::read_to_string(path).unwrap()
                    }
                    // No group specified, return empty string to remove the { group ... } block
                    None => String::new()
                }
            })
            .to_string();

        // Create previous page link(s)
        let (prev_title, prev_path) = match prev {
            Some(prev) => {
                // Previous page exists, return its title and path
                let relative_path = self.make_relative_link(&prev.path);
                ( prev.title, relative_path )
            },
            None => {
                // No previous page
                // Title: empty string; link href: "#" (as per spec, to avoid empty hrefs)
                ( String::new(), String::from("#") )
            },
        };
        self.contents = self.replace_all(&RE_PREV_TITLE, &prev_title);
        self.contents = self.replace_all(&RE_PREV_PATH, &prev_path);

        // Create next page link(s)
        let (next_title, next_path) = match next {
            Some(next) => {
                // Next page exists, return its title and path
                let relative_path = self.make_relative_link(&next.path);
                ( next.title, relative_path )
            },
            None => {
                // No next page
                // Title: empty string; link href: "#" (as per spec, to avoid empty hrefs)
                ( String::new(), String::from("#") )
            },
        };
        self.contents = self.replace_all(&RE_NEXT_TITLE, &next_title);
        self.contents = self.replace_all(&RE_NEXT_PATH, &next_path);

        // Process { tags '<markup>' } to create tag clouds
        self.contents = RE_TAGS
            .replace_all(&self.contents, |caps: &regex::Captures| {
                // Repeat tag markup for each tag (we will perform replacements further below)
                caps.name("markup").unwrap().as_str().repeat(tags.len())
            })
            .to_string();

        // For each tag in the tag cloud, substitute the link to the tag page,
        // the font size used for the link, and the page title (tag name)
        for tag in tags {
            let relative_tag_path = self.make_relative_link(&tag.1.path);
            self.contents = self.replace(&RE_TAG_PAGE_LINK, &relative_tag_path);

            // Font size is 11 + the number of pages with this tag, with an upper limit of 18
            let link_size = 11 + tag.1.meta.len().min(7);
            self.contents = self.replace(&RE_TAG_PAGE_LINK_SIZE, &link_size.to_string());

            self.contents = self.replace(&RE_TAG_PAGE_TITLE, tag.0);
        }

        // Rewrite all links and references to be relative to this document
        self.contents = super::rewrite_local_links(&self.contents, &self.output_path, &self.root_path);
    }

    pub fn write(&self) -> Result<(), anyhow::Error> {
        // Write the processed contents to the output HTML file
        write(&self.output_path, &self.contents)
            .with_context(|| format!("Unable to write output HTML file '{}'", &self.output_path.display()))
    }

    // Helper method to make links relative to this page
    fn make_relative_link(&self, target_path: &str) -> String {
        super::make_relative_link(target_path, &self.output_path, &self.root_path)
    }

    // Process { include "<path>" } blocks
    fn process_includes(&mut self) {
        // Repeat until there are no more { include ... } matches
        loop {
            let new_contents = RE_INCLUDE.replace_all(&self.contents, |caps: &regex::Captures| {
                let path = self.get_local_include_path(caps.name("path").unwrap().as_str());
                std::fs::read_to_string(path).unwrap()
            }).to_string();

            if new_contents == self.contents {
                break;
            }

            self.contents = new_contents;
        }
    }

    fn replace(&self, regex: &regex::Regex, replacement: &str) -> String {
        regex.replace(&self.contents, replacement).to_string()
    }

    fn replace_all(&self, regex: &regex::Regex, replacement: &str) -> String {
        regex.replace_all(&self.contents, replacement).to_string()
    }

    fn get_local_include_path(&self, filename: &str) -> PathBuf {
        let relative_path = &PathBuf::from(&filename);
        let mut path = PathBuf::from(&self.root_path);
        path.push(relative_path.strip_prefix("/").unwrap_or(relative_path));
        path
    }
}
