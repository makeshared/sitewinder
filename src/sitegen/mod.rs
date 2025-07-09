mod metadata;
mod page;
mod regexes;

use anyhow::{Context, Result};
use chrono::Datelike;
use std::{collections::HashMap, collections::BTreeMap, fs::File, io::BufReader, io::Read, path::Path};
use walkdir::{DirEntry, WalkDir};
use page::{Page, TemplateSource};
use regexes::*;
use metadata::Metadata;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Define characters that need to be percent-encoded in URLs
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

// A TagPage represents a collection of pages that share a common tag.
#[derive(Clone, Debug, Default)]
pub struct TagPage {
    // Path to the tag page, relative to the web root.
    pub path: String,
    // List of pages that are associated with this tag.
    pub meta: Vec<Metadata>,
}

#[derive(Debug)]
pub struct SiteGen {
    // Absolute path to the root directory where the site files are located.
    root: std::path::PathBuf,

    // Pages organised by their metadata group. The key is the group name,
    // which can be None for pages that do not belong to any group.
    groups: HashMap<Option<String>, Vec<Page>>,

    // Tags organised in a BTreeMap for sorted access. The key is the tag name,
    // and the value is a TagPage containing the path to the page and the
    // (metadata of) all the pages associated with that tag.
    tags: BTreeMap<String, TagPage>,
    // Optional tag template file content. If present, this will be used to
    // generate tag pages that list all pages associated with each tag.
    tag_template: Option<String>,
    // Optional path to the .sgtag template file. This is used to generate the
    // tag pages, and is stored to allow generating the tag page files at the
    // same location as the template file.
    tag_template_path: Option<std::path::PathBuf>,

    // Current year, used for metadata and possibly in templates.
    // We generate this only once to ensure consistency across all pages.
    current_year: String,
}

impl SiteGen {
    pub fn new(root: &std::path::Path) -> Result<SiteGen> {
        // Ensure the root is an absolute path
        let root = if root.is_absolute() {
            root.to_path_buf()
        } else {
            std::fs::canonicalize(root)
                .with_context(|| format!("Invalid webroot path: {}", root.display()))?
        };

        // Validate the path exists and is a directory
        if !root.exists() {
            anyhow::bail!("The specified path '{}' does not exist", root.display());
        }

        if !root.is_dir() {
            anyhow::bail!("The specified path '{}' is not a directory", root.display());
        }

        Ok(SiteGen {
            root,
            groups: HashMap::new(),
            tags: BTreeMap::new(),
            tag_template: None,
            tag_template_path: None,
            current_year: chrono::Local::now().year().to_string()
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Read all templates (.sgpage files) from disk and process the metadata and contents.
        self.read_templates()?;

        // If a tag template file was found, process metadata tags and generate tag pages.
        self.process_tags()?;

        // Generate all pages based on the templates and metadata.
        // This will write the HTML files to disk.       
        self.generate_pages()
    }

    //
    // Read all templates (.sgpage files) from disk, and populate the `groups` and `tags` maps.
    //
    // The `groups` map contains Pages grouped by their metadata group, while the `tags` map
    // contains lists of pages associated with each tag.
    //
    // For performance reasons, and while we are traversing the directory tree anyway, we also
    // read the contents of the first .sgtag file we encounter (if any). This is a template
    // that will be used to generate tag pages, which are separate HTML files containing links
    // to pages associated with each tag.
    //
    fn read_templates(&mut self) -> Result<()> {
        // Walk the directory tree starting from the root path
        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| e.ok()) {
            if is_file_with_extension(&entry, ".sgpage") {
                // This is a page template file.

                // Read the entire file and process its metadata (if any).
                let path = entry.path().to_path_buf();
                let mut page = Page::new(&self.root, &path, &TemplateSource::File(), &self.current_year);
                page.process_metadata().with_context(|| format!("Unable to read page template file '{}'", &path.display()))?;

                // If the page has tags in the metadata, add them to the `tags` map.
                if let Some(tags) = page.get_metadata().tags {
                    for tag in tags {
                        let val = self.tags.entry(tag).or_default();
                        val.meta.push(page.get_metadata());
                    }
                }

                // Store the Page instance in the `groups` map.
                let group = page.get_metadata().group;
                let val = self.groups.entry(group).or_default();
                val.push(page);

            } else if is_file_with_extension(&entry, ".sgtag") && self.tag_template.is_none() {
                // This is a tag template file.

                // Read the file and store its contents for later use.
                let file = File::open(entry.path())
                    .with_context(|| format!("Unable to open tag template file '{}'", entry.path().display()))?;
                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents)
                    .with_context(|| format!("Unable to read tag template file '{}'", entry.path().display()))?;

                self.tag_template = Some(contents);
                self.tag_template_path = Some(entry.path().to_path_buf());
            }
        }
        Ok(())
    }

    // Process tags by generating a Page template in memory for each tag using the specified template.
    // Each tag page will contain a list of pages associated with that tag, sorted by date in descending order (newest first).
    // The tag template file is expected to contain placeholders for the tag name and the list of pages.
    fn process_tags(&mut self) -> Result<()> {
        let Some(tag_template) = &self.tag_template else {
            // No tag template file was found, skip tag processing
            return Ok(());
        };

        for tag in &mut self.tags {
            // For each tag, sort associated pages by date in descending order (newest first)
            let pages = &mut tag.1.meta;
            pages.sort_by(|lhs, rhs| rhs.date.partial_cmp(&lhs.date).unwrap());

            // Substitute { title } for tag name
            let mut contents = RE_TITLE
                .replace_all(tag_template, |_: &regex::Captures| {
                    tag.0.clone()
                })
                .to_string();

            // Process { pages '<a href="{ page.link }">{ page.title }</a><br>' }
            // Note: Build the repeated block for all pages in one go
            let repeated_block = if let Some(caps) = RE_PAGES.captures(&contents) {
                let link_block = caps.name("link").unwrap().as_str();
                let mut result = String::new();
                for page in pages {
                    let mut block = link_block.to_string();
                    block = RE_PAGE_LINK.replace_all(&block, &page.path).to_string();
                    block = RE_PAGE_TITLE.replace_all(&block, &page.title).to_string();
                    result.push_str(&block);
                }
                result
            } else {
                String::new()
            };
            contents = RE_PAGES.replace_all(&contents, &repeated_block).to_string();

            let mut template_path = self.tag_template_path.clone().unwrap();
            template_path.set_file_name(format!("{}.sgpage", tag.0.to_lowercase()));

            // Create Page instance referring to the newly created template
            let template_source = TemplateSource::Memory(contents);
            let mut page = Page::new(&self.root, &template_path, &template_source, &self.current_year);
            page.process_metadata()?;

            tag.1.path = page.get_metadata().path;

            let val = self.groups.entry(None).or_default();
            val.push(page);
        }

        Ok(())
    }

    fn generate_pages(&mut self) -> Result<()> {
        for group in &mut self.groups {
            let pages = group.1;

            if group.0.is_none() {
                // These are pages that do not belong to any group.
                // They are generated without any grouping or prev/next links.
                for page in pages {
                    page.generate(None, None, &self.tags)?;
                }
                continue;
            }

            // For pages that belong to a group, we need to generate prev/next links.
            // Sort pages by date in ascending order (oldest first) to establish the correct order
            // for prev/next links.
            pages.sort_by(|a, b| a.get_metadata().date.partial_cmp(&b.get_metadata().date).unwrap());

            for i in 0..pages.len() {

                // Get metadata for the previous page, if any
                let prev = if i > 0 {
                    Some(pages[i - 1].get_metadata())
                } else {
                    None
                };

                // Get metadata for the next page, if any
                let next = if i + 1 < pages.len() {
                    Some(pages[i + 1].get_metadata())
                } else {
                    None
                };

                // Generate the page (this will write the HTML file to disk)
                pages[i].generate(prev, next, &self.tags)?;
            }
        }
        Ok(())
    }
}

// Helper function to check if a directory entry is a file with the specified extension
fn is_file_with_extension(entry: &DirEntry, extension: &str) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }
    if !entry
        .file_name()
        .to_str()
        .unwrap_or_default()
        .ends_with(extension)
    {
        return false;
    }
    true
}

// Helper function to make links relative to the current document and perform URI escaping as per the specification.
fn make_relative_link(target_path: &str, current_doc_path: &Path, root_path: &Path) -> String {
    let target = if let Some(stripped) = target_path.strip_prefix('/') {
        root_path.join(stripped)
    } else {
        Path::new(target_path).to_path_buf()
    };
    
    let current_dir = current_doc_path.parent().unwrap_or(Path::new(""));
    
    match pathdiff::diff_paths(&target, current_dir) {
        Some(rel_path) => {
            // Convert to forward slashes and URL encode each component
            let path_str = rel_path.to_string_lossy().replace("\\", "/");
            url_encode_path(&path_str)
        },
        None => url_encode_path(target_path),
    }
}

// Helper function to URL encode a path while preserving path separators
fn url_encode_path(path: &str) -> String {
    path.split('/')
        .map(|component| {
            // URL encode each path component, preserving forward slashes
            utf8_percent_encode(component, FRAGMENT).to_string()
        })
        .collect::<Vec<String>>()
        .join("/")
}

// Function to rewrite local links in HTML content
fn rewrite_local_links(html: &str, current_doc_path: &Path, root_path: &Path) -> String {
    let mut result = html.to_string();
    
    // Helper closure to check if URL should be rewritten
    let should_rewrite = |url: &str| -> bool {
        !url.starts_with("http://") && !url.starts_with("https://") && 
        !url.starts_with("//") && !url.starts_with("#") && 
        !url.starts_with("mailto:") && !url.starts_with("tel:") && 
        !url.starts_with("data:") && !url.starts_with("javascript:")
    };
    
    // Rewrite href attributes
    result = RE_LINK_HREF.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let url = &caps[2];
        let after = &caps[3];
        
        if should_rewrite(url) {
            let relative_url = make_relative_link(url, current_doc_path, root_path);
            format!("{}{}{}", before, relative_url, after)
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    // Rewrite src attributes
    result = RE_LINK_SRC.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let url = &caps[2];
        let after = &caps[3];
        
        if should_rewrite(url) {
            let relative_url = make_relative_link(url, current_doc_path, root_path);
            format!("{}{}{}", before, relative_url, after)
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    // Rewrite data attributes
    result = RE_LINK_DATA.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let url = &caps[2];
        let after = &caps[3];
        
        if should_rewrite(url) {
            let relative_url = make_relative_link(url, current_doc_path, root_path);
            format!("{}{}{}", before, relative_url, after)
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    // Rewrite poster attributes
    result = RE_LINK_POSTER.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let url = &caps[2];
        let after = &caps[3];
        
        if should_rewrite(url) {
            let relative_url = make_relative_link(url, current_doc_path, root_path);
            format!("{}{}{}", before, relative_url, after)
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    // Rewrite action attributes
    result = RE_LINK_ACTION.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let url = &caps[2];
        let after = &caps[3];
        
        if should_rewrite(url) {
            let relative_url = make_relative_link(url, current_doc_path, root_path);
            format!("{}{}{}", before, relative_url, after)
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    // Rewrite srcset attributes (handle comma-separated URLs)
    result = RE_LINK_SRCSET.replace_all(&result, |caps: &regex::Captures| {
        let before = &caps[1];
        let srcset = &caps[2];
        let after = &caps[3];
        
        let rewritten_srcset = srcset
            .split(',')
            .map(|entry| {
                let parts: Vec<&str> = entry.split_whitespace().collect();
                if let Some(url) = parts.first() {
                    if should_rewrite(url) {
                        let relative_url = make_relative_link(url, current_doc_path, root_path);
                        if parts.len() > 1 {
                            format!("{} {}", relative_url, parts[1..].join(" "))
                        } else {
                            relative_url
                        }
                    } else {
                        entry.to_string()
                    }
                } else {
                    entry.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("{}{}{}", before, rewritten_srcset, after)
    }).to_string();
    
    result
}
