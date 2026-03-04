use clap::Parser;
use once_cell::sync::Lazy;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser as MdParser, Tag, TagEnd};
use regex::Regex;
use std::collections::HashMap;
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

    /// 为所有块级公式自动添加编号
    #[arg(short = 'n', long = "number-equations")]
    number_equations: bool,
}

/// 预编译正则表达式
/// 这类似 C++ 中静态编译的正则模式，在程序生命周期内只编译一次
/// 块级公式：$$...$$，使用 [^$]+? 非贪婪匹配，确保内部不包含 $ 字符
/// 这样可以避免错位匹配：将 "$$ 公式A $$ 普通文本 $$ 公式B $$" 正确拆分为三个部分
static BLOCK_MATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\$([^$]+?)\$\$").unwrap());
/// 行内公式：$...$，排除包含 $ 或换行的内容
/// 注意：必须在所有 $$ 都被替换为占位符后再执行此匹配
static INLINE_MATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$([^$\n]+)\$").unwrap());

/// 预处理阶段：从 Markdown 中提取数学公式并替换为占位符
/// 这类似 C++ 编译器的预处理阶段，在正式解析前展开宏
///
/// 返回值：(处理后的文本, 数学公式HashMap)
/// - 处理后的文本：将公式替换为 {{MATH0}} 占位符（不含下划线，避免被解析为斜体）
/// - HashMap：key 为占位符，value 为 (原始公式内容, is_block) 元组
///   - is_block: true 表示块级公式 $$...$$，false 表示行内公式 $...$
fn pre_process_math(markdown: &str) -> (String, HashMap<String, (String, bool)>) {
    let mut math_tokens: HashMap<String, (String, bool)> = HashMap::new();
    let mut token_counter = 0;
    let mut processed = markdown.to_string();

    // 第一步：先处理块级公式 $$...$$
    // 必须先处理块级公式，再处理行内公式，避免冲突
    processed = BLOCK_MATH_REGEX
        .replace_all(&processed, |caps: &regex::Captures| {
            let math_content = &caps[1];
            let token = format!("{{MATH{}}}", token_counter);
            token_counter += 1;
            // 显式标记为块级公式
            math_tokens.insert(token.clone(), (math_content.to_string(), true));
            token
        })
        .to_string();

    // 第二步：处理行内公式 $...$
    processed = INLINE_MATH_REGEX
        .replace_all(&processed, |caps: &regex::Captures| {
            let math_content = &caps[1];
            let token = format!("{{MATH{}}}", token_counter);
            token_counter += 1;
            // 显式标记为行内公式
            math_tokens.insert(token.clone(), (math_content.to_string(), false));
            token
        })
        .to_string();

    (processed, math_tokens)
}

/// 将数学公式转换为 LaTeX
/// 处理 align* -> aligned 替换，并添加对应的 $ 或 $$ 包裹
fn convert_math_to_latex(math_content: &str, is_block: bool, number_equations: bool) -> String {
    // 预处理：移除公式内部的空行（连续换行符）以避免 LaTeX 编译错误
    // 使用正则将连续两个以上换行符替换为一个（移除空行）
    static MULTIPLE_NEWLINES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{2,}").unwrap());
    let cleaned = MULTIPLE_NEWLINES.replace_all(math_content, "\n");

    // 将 align* 替换为 aligned 避免与 $$ 嵌套冲突
    // 但如果用户已经明确使用了无星号的 align 或 equation，则不替换
    let processed = cleaned
        .replace("\\begin{align*}", "\\begin{aligned}")
        .replace("\\end{align*}", "\\end{aligned}")
        .replace("\\begin{align}", "\\begin{aligned}")
        .replace("\\end{align}", "\\end{aligned}");

    if is_block {
        let trimmed = processed.trim();

        // 检查用户是否已经明确指定了无星号的公式环境
        let has_explicit_numbered =
            trimmed.starts_with("\\begin{align}") || trimmed.starts_with("\\begin{equation}");

        if has_explicit_numbered {
            // 用户明确指定了编号格式，保留原样
            trimmed.to_string()
        } else if number_equations {
            // 全局开启编号，使用 equation 环境
            format!("\\begin{{equation}}\n{}\n\\end{{equation}}", trimmed)
        } else {
            // 默认使用 equation* 环境，便于用户后续手动添加编号（只需删除星号）
            format!("\\begin{{equation*}}\n{}\n\\end{{equation*}}", trimmed)
        }
    } else {
        format!("${}$", processed)
    }
}

/// LaTeX 文档模板
fn latex_template(content: &str) -> String {
    format!(
        r#"\documentclass{{ctexart}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\usepackage{{amsfonts}}
\usepackage{{xcolor}}
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

/// 匹配数学公式占位符的正则表达式
/// 在程序生命周期内只编译一次，类似 C++ 中静态编译的正则模式
static MATH_TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\{MATH\d+\})").unwrap());

/// 将 Markdown 事件转换为 LaTeX
/// 这类似于 C++ 中的 Visitor 模式
fn convert_markdown_to_latex(
    markdown: &str,
    math_tokens: &HashMap<String, (String, bool)>,
    number_equations: bool,
) -> String {
    // pulldown-cmark 使用默认选项，不需要 ENABLE_MATH
    // 因为数学公式已经在预处理阶段提取并替换为占位符了
    let parser = MdParser::new_ext(markdown, Options::all());

    let mut latex_content = String::new();
    let mut in_code_block = false;
    // 记录当前列表类型：Some(true) = 有序列表(enumerate)，Some(false) = 无序列表(itemize)
    let mut list_type_stack: Vec<bool> = Vec::new();

    for event in parser {
        match event {
            // 开始标签
            Event::Start(Tag::Heading { level, .. }) => {
                latex_content.push('\n');
                let latex_tag = match level {
                    HeadingLevel::H1 => "section",
                    HeadingLevel::H2 => "subsection",
                    HeadingLevel::H3 => "subsubsection",
                    HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => "paragraph",
                };
                latex_content.push_str(&format!("\\{}{{", latex_tag));
            }
            // 结束标签
            Event::End(TagEnd::Heading(_)) => {
                latex_content.push_str("}\n\n");
            }
            // 代码块
            Event::Start(Tag::CodeBlock(_kind)) => {
                in_code_block = true;
                latex_content.push_str("\n\\begin{verbatim}\n");
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                latex_content.push_str("\\end{verbatim}\n\n");
            }
            // 文本内容
            Event::Text(text) => {
                if in_code_block {
                    latex_content.push_str(&text);
                } else {
                    // 检查是否包含数学公式占位符 {{MATHN}}
                    if text.contains("{MATH") {
                        // 使用预编译的正则找出所有占位符并替换
                        let mut result = String::new();
                        let mut last_end = 0;

                        for cap in MATH_TOKEN_REGEX.captures_iter(&text) {
                            let full_match = cap.get(0).unwrap();
                            let token = cap.get(1).unwrap().as_str();

                            // 对占位符之前的普通文本进行转义
                            if full_match.start() > last_end {
                                let before = &text[last_end..full_match.start()];
                                result.push_str(&escape_latex(before));
                            }

                            // 替换占位符为数学公式
                            // 从 HashMap 中直接解包出 (公式内容, is_block) 元组
                            if let Some((math_content, is_block)) = math_tokens.get(token) {
                                result.push_str(&convert_math_to_latex(
                                    math_content,
                                    *is_block,
                                    number_equations,
                                ));
                            }

                            last_end = full_match.end();
                        }

                        // 转义剩余的普通文本
                        if last_end < text.len() {
                            result.push_str(&escape_latex(&text[last_end..]));
                        }

                        latex_content.push_str(&result);
                    } else {
                        // 普通文本，直接转义
                        latex_content.push_str(&escape_latex(&text));
                    }
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
            // 行内代码
            Event::Code(code) => {
                let escaped = escape_latex(&code);
                latex_content.push_str(&format!("\\texttt{{{}}}", escaped));
            }
            // 段落
            Event::SoftBreak | Event::HardBreak => {
                latex_content.push(' ');
            }
            // 段落结束：添加空行确保 LaTeX 段落间距
            Event::End(TagEnd::Paragraph) => {
                latex_content.push_str("\n\n");
            }
            // 列表：区分有序和无序
            // Tag::List(Some(_)) = 有序列表 (1. 2. 3.)
            // Tag::List(None) = 无序列表 (- * +)
            Event::Start(Tag::List(start)) => {
                let is_ordered = start.is_some();
                list_type_stack.push(is_ordered);
                if is_ordered {
                    latex_content.push_str("\n\\begin{enumerate}\n");
                } else {
                    latex_content.push_str("\n\\begin{itemize}\n");
                }
            }
            Event::End(TagEnd::List(_)) => {
                let is_ordered = list_type_stack.pop().unwrap_or(false);
                if is_ordered {
                    latex_content.push_str("\\end{enumerate}\n\n");
                } else {
                    latex_content.push_str("\\end{itemize}\n\n");
                }
            }
            Event::Start(Tag::Item) => {
                latex_content.push_str("\\item ");
            }
            Event::End(TagEnd::Item) => {
                latex_content.push('\n');
            }
            // 引用块 (Blockquote)
            Event::Start(Tag::BlockQuote(_)) => {
                latex_content.push_str("\n\\begin{quote}\n");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                latex_content.push_str("\n\\end{quote}\n\n");
            }
            // 链接
            Event::Start(Tag::Link { .. }) => {}
            Event::End(TagEnd::Link) => {}
            // 其他事件：忽略
            _ => {}
        }
    }

    latex_content
}

/// 转义 LaTeX 特殊字符
fn escape_latex(text: &str) -> String {
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

    // 预处理：提取数学公式并替换为占位符
    let (processed_markdown, math_tokens) = pre_process_math(&markdown_content);

    // 转换为 LaTeX
    let latex_content =
        convert_markdown_to_latex(&processed_markdown, &math_tokens, args.number_equations);

    // 包裹文档模板
    let full_document = latex_template(&latex_content);

    // 确定输出路径
    let output_path = if let Some(output_arg) = &args.output {
        Path::new(output_arg).to_path_buf()
    } else {
        let mut output = args.input.clone();
        if let Some(pos) = output.rfind(".md") {
            output.replace_range(pos..pos + 3, ".tex");
        } else {
            output.push_str(".tex");
        }
        PathBuf::from(output)
    };

    // 写入输出文件
    let mut file = fs::File::create(&output_path).expect("无法创建输出文件");
    file.write_all(full_document.as_bytes())
        .expect("无法写入输出文件");

    println!("转换成功！LaTeX 文件已保存至：{}", output_path.display());
}
