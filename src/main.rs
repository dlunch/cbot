use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let client = Client::new();

    let req = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(1024u16)
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a helpful assistant.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("Can you explain how to cook oil pasta in detail?")
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(req).await?;

    for choice in response.choices {
        println!("{}: Role: {}  Content: {}", choice.index, choice.message.role, choice.message.content);
    }

    Ok(())
}
