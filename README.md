# md2tex

一个将 Markdown 文档转换为中文 LaTeX (ctexart) 源码的命令行工具。

[English Version](./README-en.md)

## 项目简介

本项目是一次 **Vibe Coding** 探索实验——由 AI 辅助完成，从想法到代码的高效转化。项目基于 Rust 语言实现，利用 `pulldown-cmark` 解析 Markdown AST，并输出可直接编译为 PDF 的 LaTeX 源码。

> 所谓 "Vibe Coding"，是指在 AI 辅助下，开发者更像是在"调教"代码而非传统意义上的编写——用直觉和意图驱动开发，让代码生成紧随思维流动。

## 核心特性

- **Markdown → LaTeX 转换**：支持标题、加粗、斜体、行内代码、代码块、列表等常用语法
- **数学公式支持**：原生支持 `$...$` 行内公式和 `$$...$$` 块级公式，自动转换为 `equation*` 环境
- **公式编号开关**：支持 `-n/--number-equations` 参数全局启用公式编号，也可手动编辑 `.tex` 文件删除星号添加编号
- **中文优化**：自动套用 `ctexart` 模板，中文多级标题使用 `\chinese{}` 编号
- **单遍转义**：自定义转义算法，避免链式替换导致的二次转义污染
- **块级格式化**：代码块和列表环境自动前后空行，提升 LaTeX 源码可读性
- **零运行时依赖**：编译为单个可执行文件，无额外依赖

## 快速开始

### 前置要求

- Rust 1.70+ (通过 [rustup](https://rustup.rs/) 安装)
- (可选) [Tectonic](https://tectonic-typesetting.github.io/) 用于直接编译 PDF

### 运行方式

```bash
# 克隆项目
git clone https://github.com/Boranstars/md2tex.git
cd md2tex

# 开发模式运行
cargo run -- your_document.md

# 指定输出文件
cargo run -- your_document.md output.tex

# 全局启用公式编号
cargo run -- your_document.md -n

# 编译发布版本
cargo build --release
./target/release/md2tex your_document.md
```

### 编译为 PDF

```bash
# 使用 tectonic（推荐）
tectonic your_document.tex

# 或使用 pdflatex
pdflatex your_document.tex
```

## 支持的 Markdown 语法

| Markdown | LaTeX 输出 |
|----------|-----------|
| `# 标题` | `\section{标题}` |
| `## 标题` | `\subsection{标题}` |
| `**加粗**` | `\textbf{加粗}` |
| `*斜体*` | `\textit{斜体}` |
| `` `code` `` | `\texttt{code}` |
| ` ```rust ... ``` ` | `\begin{verbatim}...\end{verbatim}` |
| `- item` | `\begin{itemize}\item item\end{itemize}` |
| `$E=mc^2$` | `$E=mc^2$` (行内公式) |
| `$$E=mc^2$$` | `\begin{equation*}...\end{equation*}` (块级公式) |

## 技术栈

- **语言**：Rust (stable)
- **CLI**：clap (derive 模式)
- **解析**：pulldown-cmark
- **PDF**：Tectonic / pdfLaTeX

## 未来路线图

- [x] **数学公式原生穿透**：支持 `$...$` 和 `$$...$$` 语法，LaTeX 公式直接透传
- [ ] **自动图片渲染**：支持 `![alt](path/to/image.png)` 自动转换为 `\includegraphics{}`
- [ ] **表格支持**：解析 Markdown 表格转换为 LaTeX `tabular` 环境
- [ ] **更多输出格式**：支持 `ctexbook`、`article` 等不同模板
- [ ] **配置化**：支持 TOML 配置文件自定义模板和行为

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！这是一次 Vibe Coding 实验，代码风格和实现可能在探索中不断演进。
