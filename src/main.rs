use std::io::Write;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessageContentPartImage,
        ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestUserMessageArgs,
        ChatCompletionRequestUserMessageContent,
        ChatCompletionRequestUserMessageContentPart,
        CreateChatCompletionRequestArgs,
        ImageDetail, ImageUrlArgs,
    },
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use clap::Parser;
use std::{env, fs, path::Path};

#[derive(Parser)]
#[command(about = "Doubao seed skill - vision chat")]
struct Args {
    /// API key (falls back to ARK_API_KEY env var)
    #[arg(long)]
    api_key: Option<String>,

    /// Model ID (falls back to ARK_MODEL env var)
    #[arg(long)]
    model: Option<String>,

    /// API base URL (falls back to ARK_BASE_URL env var)
    #[arg(long)]
    base_url: Option<String>,

    /// Image URL or local file path (falls back to IMAGE_URL env var)
    #[arg(long)]
    image_url: Option<String>,

    /// Prompt text (falls back to PROMPT env var)
    #[arg(long)]
    prompt: Option<String>,

    /// Output file path to write the result (falls back to OUTPUT_FILE env var)
    #[arg(long)]
    output: Option<String>,
}

fn resolve_image_url(image: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(image);
    if path.exists() && path.is_file() {
        let bytes = fs::read(path)?;
        let b64 = STANDARD.encode(&bytes);
        let mime = match path.extension().and_then(|e| e.to_str()) {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            _ => "image/jpeg",
        };
        Ok(format!("data:{};base64,{}", mime, b64))
    } else {
        Ok(image.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api_key = args.api_key
        .or_else(|| env::var("ARK_API_KEY").ok())
        .expect("api_key required (--api-key or ARK_API_KEY)");

    let model = args.model
        .or_else(|| env::var("ARK_MODEL").ok())
        .unwrap_or_else(|| "doubao-seed-2-0-lite-260215".to_string());

    let base_url = args.base_url
        .or_else(|| env::var("ARK_BASE_URL").ok())
        .unwrap_or_else(|| "https://ark.cn-beijing.volces.com/api/v3".to_string());

    let image = args.image_url
        .or_else(|| env::var("IMAGE_URL").ok())
        .unwrap_or_else(|| {
            "https://ark-project.tos-cn-beijing.volces.com/doc_image/ark_demo_img_1.png".to_string()
        });

    let prompt = args.prompt
        .or_else(|| env::var("PROMPT").ok())
        .unwrap_or_else(|| "你看见了什么？".to_string());

    let output_file = args.output
        .or_else(|| env::var("OUTPUT_FILE").ok());

    let image_url = resolve_image_url(&image)?;

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);

    let client = Client::with_config(config);

    let image_part: ChatCompletionRequestUserMessageContentPart =
        ChatCompletionRequestMessageContentPartImage {
            image_url: ImageUrlArgs::default()
                .url(image_url)
                .detail(ImageDetail::Auto)
                .build()?,
        }
        .into();

    let text_part: ChatCompletionRequestUserMessageContentPart =
        ChatCompletionRequestMessageContentPartText { text: prompt }.into();

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(ChatCompletionRequestUserMessageContent::Array(vec![
                image_part,
                text_part,
            ]))
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await?;

    let mut result = String::new();
    for choice in &response.choices {
        if let Some(content) = &choice.message.content {
            result.push_str(content);
            result.push('\n');
        }
    }

    print!("{}", result);
    std::io::stdout().flush()?;

    if let Some(path) = output_file {
        fs::write(&path, &result)?;
    }

    Ok(())
}
