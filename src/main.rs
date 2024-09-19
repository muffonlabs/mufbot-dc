use std::sync::Arc;

use dotenv::dotenv;
use poise::{framework, serenity_prelude::{self, futures::lock::Mutex}};
use sqlite::{Connection, State};

struct BuildQueue {
    conn: Connection,
}

impl BuildQueue {
    fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS rollout (
                version TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                approvals INTEGER NOT NULL,
                rejections INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
        )?;
        Ok(Self { conn })
    }

    fn queue_build(&self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare("INSERT INTO rollout (version, status, approvals, rejections, created_at) VALUES (?, ?, ?, ?, ?)")?;
        stmt.bind((1, version))?;
        stmt.bind((2, "pending"))?;
        stmt.bind((3, 0))?;
        stmt.bind((4, 0))?;
        stmt.bind((5, chrono::Local::now().to_string().as_str()))?;
        stmt.next()?;
        Ok(())
    }

    fn get_builds(&self) -> Vec<String> {
        let mut builds = vec![];
        let mut stmt = self.conn.prepare("SELECT * FROM rollout").expect("failed to prepare statement");
        while let Ok(State::Row) = stmt.next() {
            let version: String = stmt.read(0).expect("failed to read version");
            let status: String = stmt.read(1).expect("failed to read status");
            let approvals: i64 = stmt.read(2).expect("failed to read approvals");
            let rejections: i64 = stmt.read(3).expect("failed to read rejections");
            let created_at: String = stmt.read(4).expect("failed to read created_at");
            builds.push(format!("{} {} {} {} {}", version, status, approvals, rejections, created_at));
        }
        builds
    }
}

struct Data {
    build_queue: Arc<Mutex<BuildQueue>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity_prelude::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn rollout(
    ctx: Context<'_>,
    #[description = "Version to roll out"] version: String,
) -> Result<(), Error> {
    // Auth check
    let guild_id = std::env::var("MUFFON_GUILD_ID").expect("missing MUFFON_GUILD_ID");
    let role_id = std::env::var("ROLLOUT_GROUP_ID").expect("missing ROLLOUT_GROUP_ID");
    let guild = ctx.http().get_guild(guild_id.parse().unwrap()).await?;
    let role = guild.roles.get(&role_id.parse().unwrap()).unwrap();
    if !ctx.author().has_role(ctx.http(), &guild, role).await? {
        ctx.say("You don't have permission to use this command").await?;
        return Ok(());
    }

    // Queue build
    let build_queue = ctx.data().build_queue.lock().await;
    build_queue.queue_build(&version).expect("failed to queue build");
    drop(build_queue);
    
    let response = format!("Rollout of version {} started", version);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn buildlist(ctx: Context<'_>) -> Result<(), Error> {
    // Auth check
    let guild_id = std::env::var("MUFFON_GUILD_ID").expect("missing MUFFON_GUILD_ID");
    let role_id = std::env::var("ROLLOUT_GROUP_ID").expect("missing ROLLOUT_GROUP_ID");
    let guild = ctx.http().get_guild(guild_id.parse().unwrap()).await?;
    let role = guild.roles.get(&role_id.parse().unwrap()).unwrap();
    if !ctx.author().has_role(ctx.http(), &guild, role).await? {
        ctx.say("You don't have permission to use this command").await?;
        return Ok(());
    }
    let mut response = String::from("```");
    response.push_str("Version Status Approvals Rejections Created At\n");
    let build_queue = ctx.data().build_queue.lock().await;
    for build in build_queue.get_builds() {
        response.push_str(&build);
        response.push('\n');
    }
    drop(build_queue);
    response.push_str("```");
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let build_queue = Arc::new(Mutex::new(BuildQueue::new("muffon.db").expect("Failed to create BuildQueue")));
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    println!("starting client");
    let intents = serenity_prelude::GatewayIntents::non_privileged();
    let framework = framework::Framework::builder()
        .options(poise::FrameworkOptions { commands: vec![age(), rollout(), buildlist()], ..Default::default() })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    build_queue: build_queue.clone(),
                })
            })
        })
        .build();
    let client = serenity_prelude::Client::builder(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.expect("failed to start client");
    println!("client started");
}
