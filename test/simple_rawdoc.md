# 测试转换

## 测试标题
这是一个测试文档，用于验证 Markdown 转 LaTeX 的转换功能。

这是一个段落，包含一些**加粗**和*斜体*文本，以及一个行内代码示例 `let x = 10;`。
```rust
fn main() {
    println!("Hello, world!");
}
```

这是行内代码示例：`let x = 10;`，应该被正确转换为 LaTeX 的 `\texttt{let x = 10;}`。

这是一个无序列表：
- 项目一
- 项目二
- 项目三

这是一个有序列表：
1. 第一项
2. 第二项
3. 第三项

## 数学公式测试

这是行内公式：$E=mc^2$，应该原样保留。

这是块级公式：

$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

这是第二个块级公式：
$$
f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(0)}{n!} x^n
$$

给公式添加颜色：
$$
\textcolor{red}{E=mc^2}
$$


这是一个有编号的公式：
```latex
\begin{equation}
a^2 + b^2 = c^2
\end{equation}
```

公式中的特殊字符如 `^`, `_`, `{`, `}` 不应被转义。

## 图片测试


![项目印象图](Gemini_Generated_Image_a40x9qa40x9qa40x.png)
