# md2tex

[![CI](https://github.com/Boranstars/md2tex/actions/workflows/ci.yml/badge.svg)](https://github.com/Boranstars/md2tex/actions)
[![Release](https://img.shields.io/github/v/release/Boranstars/md2tex)](https://github.com/Boranstars/md2tex/releases/latest)
[![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-blue)](https://github.com/Boranstars/md2tex/releases)

A command-line tool that converts Markdown documents to Chinese LaTeX (ctexart) source code.

[中文版本](./README.md)

## Project Overview

This project is a **Vibe Coding** exploration experiment—AI-assisted efficient transformation from idea to code. Built with Rust, it uses `pulldown-cmark` to parse Markdown AST and outputs LaTeX source code ready for PDF compilation.

> "Vibe Coding" means developers, with AI assistance, are more like "tuning" code than traditionally writing it—driving development with intuition and intent, letting code generation follow the flow of thought.

## Features

- **Markdown → LaTeX Conversion**: Supports headings, bold, italic, inline code, code blocks, lists, and more
- **Math Formula Support**: Native support for `$...$` inline and `$$...$$` block formulas, auto-converted to `equation*` environment
- **Equation Numbering Toggle**: Supports `-n/--number-equations` flag for global equation numbering, or manually edit `.tex` file by removing the asterisk
- **Chinese Optimization**: Auto-applies `ctexart` template with `\chinese{}` numbering for multi-level Chinese headings
- **Single-Pass Escaping**: Custom escaping algorithm avoids double-escaping pollution from chained replacements
- **Block-Level Formatting**: Code blocks and list environments automatically have blank lines before/after for better LaTeX source readability
- **Zero Runtime Dependencies**: Compiles to a single executable with no additional dependencies

## Installation

### Build from Source

Prerequisites:
- Rust 1.70+ (installed via [rustup](https://rustup.rs/))

```bash
# Clone the project
git clone https://github.com/Boranstars/md2tex.git
cd md2tex

# Run in development mode
cargo run -- your_document.md

# Build release version
cargo build --release
./target/release/md2tex your_document.md
```

### Pre-built Binaries

Download pre-built binaries for your platform from the [Releases](https://github.com/Boranstars/md2tex/releases) page.

## Usage

```bash
# Basic usage: convert Markdown file
md2tex your_document.md

# Specify output file
md2tex your_document.md output.tex

# Enable global equation numbering
md2tex your_document.md -n
md2tex your_document.md --number-equations

# View help
md2tex --help
```

### Compile to PDF

```bash
# Using tectonic (recommended)
tectonic your_document.tex

# Or using pdflatex
pdflatex your_document.tex
```

## Supported Markdown Syntax

| Markdown | LaTeX Output |
|----------|---------------|
| `# Heading` | `\section{Heading}` |
| `## Heading` | `\subsection{Heading}` |
| `**bold**` | `\textbf{bold}` |
| `*italic*` | `\textit{italic}` |
| `` `code` `` | `\texttt{code}` |
| ` ```rust ... ``` ` | `\begin{verbatim}...\end{verbatim}` |
| `- item` | `\begin{itemize}\item item\end{itemize}` |
| `$E=mc^2$` | `$E=mc^2$` (inline) |
| `$$E=mc^2$$` | `\begin{equation*}...\end{equation*}` (block) |

## Tech Stack

- **Language**: Rust (stable)
- **CLI**: clap (derive mode)
- **Parser**: pulldown-cmark
- **PDF**: Tectonic / pdfLaTeX

## Roadmap

- [x] **Math Formula Pass-through**: Support `$...$` and `$$...$$` syntax, LaTeX formulas passed through directly
- [ ] **Auto Image Rendering**: Support `![alt](path/to/image.png)` auto-converted to `\includegraphics{}`
- [ ] **Table Support**: Parse Markdown tables to LaTeX `tabular` environment
- [ ] **More Output Formats**: Support `ctexbook`, `article`, and other templates
- [ ] **Configuration**: Support TOML config file for template and behavior customization

## License

MIT License - See [LICENSE](LICENSE) file

## Contributing

Welcome to submit Issues and Pull Requests! This is a Vibe Coding experiment, code style and implementation may evolve during exploration.
