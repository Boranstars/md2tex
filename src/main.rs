use clap::Parser;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser as MdParser, Tag, TagEnd};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// 命令行参数解析器
#[derive(Parser, Debug)]
#[command(name = "md2tex")]
#[command(about = "将 Markdown 文件转换为 LaTeX 源码")]
struct Args {
    /// 输入的 Markdown 文件路径
    #[arg(index = 1)]
    input: String,

    /// 输出的 LaTeX 文件路径（可选，默认与输入文件同名）
    #[arg(index = 2)]
    output: Option<String>,
}

/// LaTeX 文档模板
fn latex_template(content: &str) -> String {
    format!(
        r#"\documentclass{{ctexart}}
\pagestyle{{plain}}
\ctexset{{
    section = {{name = {{,、}}, number = \chinese{{section}}}},
    subsection = {{name = {{（,）}}, number = \chinese{{subsection}}}},
    subsubsection = {{number = \arabic{{subsubsection}}}}
}}

\begin{{document}}

{}

\end{{document}}
"#,
        content
    )
}

/// 将 Markdown 事件转换为 LaTeX
/// 这类似于 C++ 中的 Visitor 模式：pulldown-cmark 遍历 AST 时，
/// 我们"访问"每个节点（Event），然后将其"转换"为对应的 LaTeX 字符串。
/// 这里的 String 使用 Rust 的 Ownership 语义 —— 每次转换都创建新的字符串，
/// 类似于 C++ 中每次返回 std::string（而不是返回引用）。
fn convert_markdown_to_latex(markdown: &str) -> String {
    // pulldown-cmark 使用 Iterator 模式遍历 AST
    // 这里的 parser 是一个"懒迭代器"，类似 C++ 中的范围-for 循环
    let parser = MdParser::new_ext(markdown, Options::all());

    let mut latex_content = String::new();
    let mut in_code_block = false;

    // 这里的 for 循环类似 C++ 范围-for：
    // for (const auto& event : parser) { ... }
    for event in parser {
        match event {
            // 开始标签：类似于 C++ 中的 Tag Start 事件
            Event::Start(Tag::Heading { level, .. }) => {
                // 使用 C++ 枚举对比的方式匹配级别
                // # 对应 level = H1, ## 对应 H2, ...
                let latex_tag = match level {
                    HeadingLevel::H1 => "section",
                    HeadingLevel::H2 => "subsection",
                    HeadingLevel::H3 => "subsubsection",
                    HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => "paragraph",
                };
                // 直接写入 LaTeX 命令，不要转义反斜杠
                latex_content.push_str(&format!("\\{}{{", latex_tag));
            }
            // 结束标签：对应 Markdown 的 ## 结束位置
            Event::End(TagEnd::Heading(_)) => {
                latex_content.push_str("}\n\n");
            }
            // 代码块开始
            Event::Start(Tag::CodeBlock(_kind)) => {
                in_code_block = true;
                // 代码语言信息暂时忽略（可后续用于 listings 宏包）
                // 前面加空行分隔
                latex_content.push_str("\n\\begin{verbatim}\n");
            }
            // 代码块结束
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                // 后面加空行分隔
                latex_content.push_str("\\end{verbatim}\n\n");
            }
            // 文本内容
            Event::Text(text) => {
                if in_code_block {
                    // 代码块内：直接保留原始文本
                    // 这类似 C++ 中直接 push_back 字符
                    latex_content.push_str(&text);
                } else {
                    // 普通文本：处理基本转义
                    // 在 C++ 中我们可能需要手动转义 &, %, $, #, _, {, }
                    let escaped = escape_latex(&text);
                    latex_content.push_str(&escaped);
                }
            }
            // 强调/斜体
            Event::Start(Tag::Emphasis) => {
                latex_content.push_str("\\textit{");
            }
            Event::End(TagEnd::Emphasis) => {
                latex_content.push('}');
            }
            // 粗体
            Event::Start(Tag::Strong) => {
                latex_content.push_str("\\textbf{");
            }
            Event::End(TagEnd::Strong) => {
                latex_content.push('}');
            }
            // 行内代码：pulldown-cmark 使用 Event::Code 表示反引号包围的代码
            Event::Code(code) => {
                // 行内代码用 \texttt{} 包裹，并转义特殊字符
                let escaped = escape_latex(&code);
                latex_content.push_str(&format!("\\texttt{{{}}}", escaped));
            }
            // 段落（pulldown-cmark 会将段落作为隐式标签）
            Event::SoftBreak | Event::HardBreak => {
                // 软换行 -> 空格，硬换行 -> \\\\
                latex_content.push(' ');
            }
            // 列表项
            Event::Start(Tag::List(_)) => {
                // 前面加空行分隔
                latex_content.push_str("\n\\begin{itemize}\n");
            }
            Event::End(TagEnd::List(_)) => {
                // 后面加空行分隔
                latex_content.push_str("\\end{itemize}\n\n");
            }
            Event::Start(Tag::Item) => {
                latex_content.push_str("\\item ");
            }
            Event::End(TagEnd::Item) => {
                latex_content.push('\n');
            }
            // 链接（暂时忽略 URL）
            Event::Start(Tag::Link { .. }) => {}
            Event::End(TagEnd::Link) => {}
            // 其他事件：忽略
            _ => {}
        }
    }

    latex_content
}

/// 转义 LaTeX 特殊字符
/// 使用单遍字符处理，避免链式 replace 导致的二次转义问题
/// 类似 C++ 中手动遍历字符数组并逐个 append 转义字符串
fn escape_latex(text: &str) -> String {
    // 预分配足够空间（每个字符最多约 16 字节的转义序列）
    let mut result = String::with_capacity(text.len() * 16);

    for c in text.chars() {
        match c {
            '\\' => result.push_str("\\textbackslash{}"),
            '&' => result.push_str("\\&"),
            '%' => result.push_str("\\%"),
            '$' => result.push_str("\\$"),
            '#' => result.push_str("\\#"),
            '_' => result.push_str("\\_"),
            '{' => result.push_str("\\{"),
            '}' => result.push_str("\\}"),
            '~' => result.push_str("\\textasciitilde{}"),
            '^' => result.push_str("\\textasciicircum{}"),
            // 其他字符直接保留（包括普通字母、数字、中文等）
            _ => result.push(c),
        }
    }

    result
}

fn main() {
    let args = Args::parse();

    // 读取输入文件
    let input_path = Path::new(&args.input);
    if !input_path.exists() {
        eprintln!("错误：输入文件不存在：{}", args.input);
        std::process::exit(1);
    }

    let markdown_content = fs::read_to_string(input_path).expect("无法读取输入文件");

    // 转换为 LaTeX
    let latex_content = convert_markdown_to_latex(&markdown_content);

    // 包裹文档模板
    let full_document = latex_template(&latex_content);

    // 确定输出路径
    // 使用 PathBuf 拥有所有权，类似 C++ 的 std::filesystem::path
    let output_path = if let Some(output_arg) = &args.output {
        Path::new(output_arg).to_path_buf()
    } else {
        // 默认将 .md 替换为 .tex
        let mut output = args.input.clone();
        if let Some(pos) = output.rfind(".md") {
            output.replace_range(pos..pos + 3, ".tex");
        } else {
            output.push_str(".tex");
        }
        PathBuf::from(output)
    };

    // 写入输出文件（使用引用，类似 C++ 中的 const&）
    let mut file = fs::File::create(&output_path).expect("无法创建输出文件");
    file.write_all(full_document.as_bytes())
        .expect("无法写入输出文件");

    println!(
        "转换成功！LaTeX 文件已保存至：{}",
        output_path.display()
    );
}
