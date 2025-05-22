use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// goImage CLI 客户端 - 用于上传图片到goImage服务
#[derive(Parser, Debug)]
#[command(name = "rustora")]
#[command(author = "NodeSeeker")]
#[command(version = "0.1.0")]
#[command(about = "用于Typora上传图片到goImage服务的CLI客户端")]
struct Args {
    /// 服务器API URL
    #[arg(short, long)]
    url: String,

    /// 要上传的图片文件路径
    #[arg(short, long)]
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ImageData {
    url: String,
    #[allow(dead_code)]
    filename: String,
    #[allow(dead_code)]
    #[serde(rename = "contentType")]
    content_type: String,
    #[allow(dead_code)]
    size: u64,
    #[allow(dead_code)]
    #[serde(rename = "uploadTime")]
    upload_time: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<ImageData>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse();
    
    // 检查文件是否存在
    if !args.file.exists() {
        anyhow::bail!("文件不存在: {:?}", args.file);
    }
    
    // 检查URL格式
    if !args.url.starts_with("http://") && !args.url.starts_with("https://") {
        anyhow::bail!("URL必须以 http:// 或 https:// 开头");
    }
    
    // 准备文件数据
    let file_name = args.file.file_name()
        .context("无法获取文件名")?
        .to_string_lossy()
        .to_string();
    
    // 打开文件
    let mut file = File::open(&args.file).await
        .context("无法打开文件")?;
    
    // 读取文件内容
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await
        .context("无法读取文件内容")?;
    
    // 猜测 MIME 类型
    let mime_type = mime_guess::from_path(&args.file)
        .first_or_octet_stream()
        .to_string();
    
    // 创建 multipart form
    let form_part = multipart::Part::bytes(buffer)
        .file_name(file_name)
        .mime_str(&mime_type)
        .context("MIME 类型无效")?;
    
    let form = multipart::Form::new()
        .part("image", form_part);
    
    // 发送请求
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("无法创建HTTP客户端")?;
    let res = client.post(&args.url)
        .multipart(form)
        .send()
        .await
        .context("上传请求失败")?;
    
    // 解析响应
    if res.status().is_success() {
        let response: ApiResponse = res.json().await
            .context("无法解析响应JSON")?;
        
        if response.success {
            if let Some(data) = response.data {
                // 只输出URL，便于集成到其他命令中
                println!("{}", data.url);
            } else {
                anyhow::bail!("成功响应中缺少数据字段");
            }
        } else {
            anyhow::bail!("上传失败: {}", response.message);
        }
    } else {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| "无法读取错误详情".to_string());
        anyhow::bail!("服务器返回错误: {} - {}", status, text);
    }
    
    Ok(())
}
