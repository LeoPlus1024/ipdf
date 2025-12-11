# pdf-rs

[中文](#chinese) | [English](README.md)

### 项目概述

`pdf-rs` 是一个用于解析 PDF 文件的 Rust 库。该项目旨在提供对 PDF 文档结构的解析功能，包括：

- PDF 版本识别
- 交叉引用表（xref）解析
- 对象解析（字典、数组、字符串等）
- 基本的 PDF 结构访问


### 关键特性

1. **PDF 版本支持**: 支持从 1.0 到 2.0 的 PDF 版本
2. **对象解析**: 解析 PDF 中的各种对象类型，包括字典、数组、字符串等
3. **交叉引用表解析**: 解析 PDF 的 xref 表以定位对象
4. **流式读取**: 使用 `Sequence` trait 实现高效的流式文件读取

### 使用示例

```rust
use std::fs::File;
use ipdf::document::PDFDocument;
use ipdf::sequence::FileSequence;

// 创建 PDF 文档解析器
let file = File::open("example.pdf")?;
let sequence = FileSequence::new(file);
let document = PDFDocument::new(sequence)?;

// 访问 PDF 版本
println!("PDF Version: {}", document.get_version());

// 获取交叉引用表
let xrefs = document.get_xref();
```

### 设计亮点

- **模块化设计**: 各个功能分离到不同模块，便于维护和扩展
- **错误处理**: 完善的错误类型系统，提供详细的错误信息
- **内存效率**: 使用流式读取避免将整个文件加载到内存
- **类型安全**: 充分利用 Rust 的类型系统保证安全性

### 当前状态

项目处于早期开发阶段，已实现基本的 PDF 解析功能，包括版本检测、xref 表解析和基本对象解析。

### 未来计划

- 完善 PDF 对象解析功能
- 添加加密 PDF 支持
- 实现更高级的 PDF 内容提取功能
- 提供更友好的 API 接口

### 构建要求

- Rust 1.5+（推荐使用最新稳定版）

### 构建步骤

```bash
cargo build
```

### 运行测试

```bash
cargo test
```