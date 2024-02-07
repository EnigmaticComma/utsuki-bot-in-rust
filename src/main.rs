use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    println!("Starting age command!");
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    println!("{} executed age command!", u.name);
    Ok(())
}

#[poise::command(slash_command)]
async fn avatar(
    ctx: Context<'_>,
    #[description = "User to get avatar of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    println!("Starting avatar command!");
    let user = user.unwrap_or_else(|| *ctx.author());
    let avatar_url = user.face().await;
    ctx.send(avatar_url).await?;
    println!("{} executed avatar command!", user.name);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting Utsuki Discord Bot");
    let token = std::env::var("DISCORD_TOKEN_UTSUKI").expect("missing DISCORD_TOKEN_UTSUKI");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    println!("Utsuki Discord Bot started!");
}