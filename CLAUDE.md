# 项目概览
本项目是一个使用 Rust 编写的命令行工具 (`md2tex`)，用于将 Markdown 语法转换为高质量的中文 LaTeX 源码，以便后续使用 Tectonic 编译为 PDF。

# 技术栈限制
1. **语言**：Rust (最新的稳定 Edition)。严格遵守 `cargo clippy` 的建议。
2. **CLI 框架**：使用 `clap` (derive 模式) 处理命令行参数（如输入文件路径、输出文件路径）。
3. **Markdown 解析**：强制使用 `pulldown-cmark` 库处理 AST（抽象语法树）的遍历和解析，不要手写正则来解析 Markdown。

# 沟通与解释规范 (面向用户的特殊要求)
1. 用户是逻辑严密的开发者，熟悉底层系统架构和现代 C++ (C++17/20/23)，但不熟悉rust。
2. 在编写或修改 Rust 代码后，必须向用户解释核心逻辑。
3. 当涉及 Rust 的特有概念（如 Ownership, Borrowing, Lifetimes, Traits）时，请务必使用 C++ 的概念（如 RAII, `std::unique_ptr`, `std::move`, 虚表, Concepts）进行类比，帮助用户快速理解。

# 目标 LaTeX 模板规范
转换器生成的 `.tex` 文件，必须套用以下基础模板架构：
1. 文档类：`\documentclass{ctexart}`
2. 页面样式：`\pagestyle{plain}` (禁止显示默认的章节页眉)
3. 自动编号修复：使用 `\ctexset{section = {name = {,、}, number = \chinese{section}}}` 规范标题。
4. Markdown 的 `# 标题` 映射为 `\section{}`，`## 标题` 映射为 `\subsection{}`。
5. Markdown 的代码块映射为 `verbatim` 环境或引入 `listings` 宏包处理。