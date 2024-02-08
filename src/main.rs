use poise::serenity_prelude as serenity;
use rand::Rng;

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

/// Command to randomize a d20 dice roll:
#[poise::command(slash_command, prefix_command)]
async fn d20(ctx: Context<'_>) -> Result<(), Error> {
    println!("Starting d20 command!");
    let roll = rand::thread_rng().gen_range(1..=20);
    ctx.say(format!("You rolled a d20 and got a {}", roll)).await?;
    println!("{} executed d20 command!", ctx.author().name);
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