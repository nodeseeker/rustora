# rustora
用于Typora的的图片上传插件，基于goImage
# Rustora

Rustora 是一个用 Rust 编写的命令行工具，用于将图片上传到 [goImage](https://github.com/nodeseeker/goImage) 服务。它提供了简单的接口，可以一次上传一张或多张图片，并返回图片的访问 URL。

## 功能特点

- 支持单张或多张图片上传
- 自动检测文件 MIME 类型
- 简洁的命令行输出，便于与其他工具集成
- 完善的错误处理和提示信息

## 编译方法

### Linux 下编译 Windows 版本

```bash
cargo clean && cargo build --release --target x86_64-pc-windows-gnu
```

### 本地编译

```bash
cargo build --release
```

## 使用方法

### 基本用法

```bash
rustora --url <服务器API地址> --files <图片文件路径>
```

### 上传单个图片

```bash
rustora --url https://image.example.com/api/v1/upload --files path/to/image.jpg
```

### 上传多个图片

```bash
rustora --url https://image.example.com/api/v1/upload --files image1.jpg image2.png image3.gif
```

或者使用 Windows 路径格式：

```bash
rustora.exe --url https://image.example.com/api/v1/upload --files "C:\path\to\image1.jpg" "C:\path\to\image2.png"
```

## 输出说明

成功上传时，程序会在标准输出中打印上传后的图片 URL。这便于将输出直接用于其他命令或脚本。

如果上传失败，错误信息会输出到标准错误流，并包含详细的错误原因。

## 注意事项

- URL 必须以 `http://` 或 `https://` 开头
- 文件必须存在且可读
- 程序支持各种图片格式，包括 JPG、PNG、GIF、WebP 等

## 依赖项

- clap: 命令行参数解析
- reqwest: HTTP 客户端
- tokio: 异步运行时
- anyhow: 错误处理
- serde: JSON 序列化/反序列化
- mime_guess: MIME 类型检测

## 许可证

General Public License v3.0
```