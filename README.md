# sitewinder

A Static Site Generator for building websites using templates and page metadata.

Static site generators build complete websites from templates and content files, creating HTML files that can be served directly by a web server without any server-side processing.

## Table of Contents
- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Template Types](#template-types)
- [Page Metadata](#page-metadata)
- [Template Blocks](#template-blocks)
- [Navigation Features](#navigation-features)
- [Links and References](#links-and-references)
- [Licence](#licence)

## Features

Sitewinder makes it easy to organise pages by groups and tags, and to generate navigation links and tag clouds automatically.

- **Simple template syntax** with curly-brace blocks
- **Easy file inclusion** and variable substitution
- **Metadata-driven page organization** (groups, tags, dates)
- **Automatic previous/next navigation** within groups
- **Tag cloud generation** with font size scaling
- **Relative link generation** that works anywhere

## Installation

You can either build from source or download a binary release.

### Build from Source
```bash
# install sitewinder in the user's ~/.local/bin folder
cargo install --path . --root ~/.local
```

### Download Release

Pre-built binaries are available for Linux, macOS, and Windows from the [releases page](https://github.com/makeshared/sitewinder/releases).

### Generate Your Site

Once you have built or downloaded the sitewinder binary, you can build your site:

```bash
# Linux / macOS
sitewinder /path/to/webroot

# Windows
sitewinder.exe C:\path\to\webroot
```

## Running the Examples

The [examples](examples/) folder demonstrates sitewinder's features. Start with the hello world example:

```bash
cargo run --example hello_world
```

Other examples to explore:
- `groups` - create links to previous/next pages
- `tags` - build tag cloud markup and pages
- `full_site` - a full-featured site

## Suggested Workflow

Use one of the [examples](examples/) as a starting point for your site.

Once the site structure has been established, create new pages by adding `.sgpage` files.

To build the site after changes have been made:

```sh
WEBROOT=/path/to/webroot

# Find and delete existing HTML files to ensure we get a clean run
find ${WEBROOT}/ -name '*.html' -type f -delete

# Run sitewinder to generate a new set of HTML files
sitewinder ${WEBROOT}

# Run HTML 'tidy' to format HTML file contents and check for markup errors
echo "tidying HTML (no errors should be seen)"
find ${WEBROOT}/ -name '*.html' -type f -print0 | sort -z | xargs -r0 \
    tidy -utf8 -indent -wrap 160 --tidy-mark no -quiet -modify "$@"
```

## Template Types

Sitewinder supports three types of input files:

| Extension | Purpose | Description |
|-----------|---------|-------------|
| `.sgpage` | Page template | Used to generate an HTML file |
| `.sgtag` | Tag page template | Structure template for tag pages |
| `.sginc` | Include file | Included in other sitewinder templates |

## Page Metadata

`.sgpage` files may contain metadata at the beginning of the file. A metadata block must be contained within double hyphens:

```
--
title: Alpine Club Trip Report 2024
author: Alex
group: Trips
date: 2024-12-16
tags: Trips, 2024, Taranaki
--
```

**Supported fields:**
- `title` - The page title
- `author` - Author name
- `group` - Category for previous/next links
- `date` - Publication date (YYYY-MM-DD format)
- `tags` - Comma-separated list of tags

All fields are optional.

## Template Blocks

Sitewinder processes blocks delimited by curly braces `{ ... }`.

### Basic Blocks
- `{ include "<path>" }` - Include another file
- `{ title }` - Insert page title from metadata
- `{ author }` - Insert author from metadata
- `{ current_year }` - Insert current calendar year, useful for copyright notices

### Navigation Blocks
- `{ group "<path>" }` - Include navigation markup for group pages
- `{ tags '<markup>' }` - Generate tag cloud navigation

## Navigation Features

### Group Navigation

Pages within the same `group` are automatically linked in chronological order based on their `date`.

**Available variables:**
- `{ prev.path }` - Path to previous page in group
- `{ prev.title }` - Title of previous page
- `{ next.path }` - Path to next page in group
- `{ next.title }` - Title of next page

**Example usage:**
```html
<!-- Place this in e.g. /group_nav.sginc -->
<nav>
  <a href="{ prev.path }">← { prev.title }</a> | 
  <a href="{ next.path }">{ next.title } →</a>
</nav>
```

**Conditional inclusion:**
```html
<!-- Only include navigation markup on pages that belong to a group -->
{ group "/group_nav.sginc" }
```

### Tag Cloud Navigation

Pages can specify tags in metadata. Sitewinder generates tag pages and tag clouds automatically.

**Tag template example (tag.sgtag):**
```html
---
title: { title }
---

{ include "/common/header.sginc" }
<div>
    { pages '<p><a href="{ page.link }">{ page.title }</a></p>' }
</div>
{ include "/common/footer.sginc" }
```

**Tag cloud example:**
```html
{ tags '<a href="{ tag.page.link }" style="font-size: { tag.page.link_size }pt;">{ tag.page.title }</a> ' }
```

**Tag cloud variables:**
- `{ tag.page.link }` - Path to tag page
- `{ tag.page.title }` - Title of tag page
- `{ tag.page.link_size }` - Font size scaled by popularity

## Links and References

Sitewinder generates relative links that work regardless of where the HTML files are stored - on a web server or in the local file system.

**Links in template files must be relative to the web root:**
```html
<!-- In template files, use root-relative paths -->
<link rel="stylesheet" href="/style.css">
```

**Generated output adapts to page location:**
```html
<!-- In /index.html -->
<link rel="stylesheet" href="style.css">

<!-- In /posts/2025-01-04.html -->
<link rel="stylesheet" href="../style.css">
```

## Licence

This project is licensed under the [MIT Licence](LICENSE). You are free to use, modify, and distribute it for any purpose.
