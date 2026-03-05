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

下面是不同大小的图片测试：

<img src="Gemini_Generated_Image_a40x9qa40x9qa40x.png" alt="图片测试1" width="200"/>

<img src="Gemini_Generated_Image_a40x9qa40x9qa40x.png" alt="图片测试2" width="300"/>

其他的html width属性测试：

<img src="Gemini_Generated_Image_a40x9qa40x9qa40x.png" alt="图片测试3" width="50%"/>

<img src="Gemini_Generated_Image_a40x9qa40x9qa40x.png" alt="图片测试4" width="100px"/>



## 链接测试
- 这是一个链接：[点击这里访问 OpenAI](https://www.openai.com)
- 这是一个链接：[点击这里访问 GitHub](https://github.com)
- 这是一个链接：[点击这里访问 Rust 官网](https://www.rust-lang.org)

## 表格测试

这是一个简单的表格：
| 左对齐 | 居中 | 右对齐 |
|:-------|:----:|-------:|
| 内容1  | 内容2 | 内容3 |
| 内容4  | 内容5 | 内容6 |  

第二个表格：
| 姓名 | 年龄 |
|------|------|
| 张三 | 25   |
| 李四 | 30   | 

复杂表格：
| 参数名称 | 符号 | 默认值 | 单位 |
| :--- | :---: | :---: | ---: |
| 采样频率 | $f_s$ | 1000 | Hz |
| 截止频率 | $\omega_c$ | 50 | rad/s |
| 阻尼比 | $\zeta$ | 0.707 | - |

带公式的表格：
| 参数 | 公式 |
|------|------|
| 频率响应 | $H(j\omega) = \frac{1}{1 + j\omega/\omega_c}$ |
| 阻尼比 | $\zeta = \frac{c}{2\sqrt{mk}}$ |

带空行的表格：
| 实验编号 | 观测现象描述 | 备注 |
| --- | --- | --- |
| EXP-01 | 系统在输入阶跃信号后出现小幅震荡，超调量约 15%。 | 正常 |
| EXP-02 | | 数据待补 |
| EXP-03 | 调整 PID 参数后，稳态误差消除，收敛速度明显加快。 | **推荐参数** |