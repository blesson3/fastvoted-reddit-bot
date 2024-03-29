#[macro_use]
extern crate log;

use config::Config;
// use log::{debug, error, info, warn};
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

mod config;
mod reddit;
mod reddit_models;

use reddit::RedditApi;

#[derive(Debug)]
struct Post
{
    pub id:              u64,
    pub source:          String,
    pub link:            String,
    pub title:           String,
    pub velocity:        String,
    pub discussion_link: String,
}

impl Post
{
    pub fn new(
        source: String,
        link: String,
        title: String,
        velocity: String,
        discussion_link: String,
    ) -> Self
    {
        // generate the id from the given inputs
        let mut hasher = DefaultHasher::new();
        format!("{}{}{}{}{}", source, link, title, velocity, discussion_link).hash(&mut hasher);
        let id = hasher.finish();

        Post {
            id,
            source,
            link,
            title,
            velocity,
            discussion_link,
        }
    }
}

fn init_log()
{
    // if `RUST_LOG` env var not set, default to info
    if let None = env::var("RUST_LOG").ok()
    {
        env::set_var("RUST_LOG", "info");
    }

    // check if debug or release build
    if cfg!(debug_assertions)
    {
        env_logger::init();
    }
    else
    {
        // syslog friendly format
        // https://github.com/env-logger-rs/env_logger/blob/master/examples/syslog_friendly_format.rs
        env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "<{}>{}: {}",
                    match record.level()
                    {
                        log::Level::Error => 3,
                        log::Level::Warn => 4,
                        log::Level::Info => 6,
                        log::Level::Debug => 7,
                        log::Level::Trace => 7,
                    },
                    record.target(),
                    record.args()
                )
            })
            .init();
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>
{
    init_log();

    // get config
    let config_dir = env::var("CONFIG_DIR").expect("environment variable CONFIG_DIR must be set");
    let config = Config::from_directory(Path::new(&config_dir));

    // check for "clear" arg
    let contains_clear_arg = env::args()
        .collect::<Vec<String>>()
        .contains(&"clear".to_string());

    if contains_clear_arg
    {
        delete_all_posts(&config).await?;
    }
    else
    {
        loop
        {
            // run
            check_and_post_content(&config).await?;

            // delay for 15 minutes
            let delay_minutes = 15;
            info!("Delay for {} minutes...", delay_minutes);
            sleep(Duration::from_secs(60 * delay_minutes)).await;
        }
    }

    Ok(())
}

async fn get_authorized_reddit_api(config: &Config) -> Result<RedditApi, Box<dyn Error>>
{
    // create new reddit api and authorize it
    let mut reddit_api = RedditApi::new();
    reddit_api
        .authorize(
            &config.username,
            &config.password,
            &config.client_id,
            &config.client_secret,
        )
        .await?;

    Ok(reddit_api)
}

async fn check_and_post_content(config: &Config) -> Result<(), Box<dyn Error>>
{
    info!("Checking for new content...");

    // uncomment to use cached version of webpage
    // let mut file = File::open("fastvoted.com.txt")?;
    // let mut html = String::new();
    // file.read_to_string(&mut html)?;

    let html = reqwest::get("https://fastvoted.com").await?.text().await?;

    let mut all_posts: Vec<Post> = vec![];

    let post_whole_re =
        Regex::new("<div class=\"post\">(.|\n)+?</div></div>").expect("post whole regex compiles");
    let post_comps_re = Regex::new("source\">([^<]+)</div>(?:.|\n)*?title\"><a target(?:.|\n)*?href=\"([^\"]*)\">([^<]*)(?:.|\n)*?text\">([^<]*)(?:.|\n)*?href=\"([^\"]*)\"").expect("post comps regex compiles");
    for cap in post_whole_re.captures_iter(&html)
    {
        let post = &cap[0];
        let comps = post_comps_re.captures_iter(post);
        for comp in comps
        {
            let source = &comp[1];
            let link = &comp[2];
            let title = &comp[3].replace("\n", ""); // remove all newlines
            let velocity = &comp[4].replace(" |", "");
            let discussion_link = &comp[5];

            trace!(
                "source: {}, link: {}, velocity: {}, discussion_link: {}",
                source,
                link,
                velocity,
                discussion_link
            );

            // escape the html entities in the title like... &#39; => '
            let title: &str = &html_escape::decode_html_entities(title);

            all_posts.push(Post::new(
                source.to_string(),
                link.to_string(),
                title.to_string(),
                velocity.to_string(),
                discussion_link.to_string(),
            ));
        }
    }

    let mut last_posted_id_string: String = "".to_string();
    if let Ok(last_posted_ids_file) = File::open(config.get_last_posted_id_filepath())
    {
        let mut posted_ids_reader = BufReader::new(&last_posted_ids_file);
        posted_ids_reader.read_line(&mut last_posted_id_string)?;
        last_posted_id_string = last_posted_id_string.replace("\n", "");
    }

    // if is empty, post everything, if not, then figure out which ones we need to post
    if !last_posted_id_string.is_empty()
    {
        debug!("last posted id = '{}'", &last_posted_id_string);
        let last_posted_id: u64 = u64::from_str_radix(&last_posted_id_string, 10)?;

        let post_position = all_posts.iter().position(|p| p.id == last_posted_id);
        debug!("last posted position: {:?}", &post_position);

        match post_position
        {
            Some(x) =>
            {
                // post all the posts before the one we last posted
                all_posts = all_posts.into_iter().take(x).collect();
            }
            None =>
            {
                // post everything
            }
        }
    }

    if all_posts.is_empty()
    {
        info!("Nothing to post.");
        return Ok(());
    }

    info!("Found {} new posts", all_posts.len());

    // create new reddit api and authorize it
    let reddit_api = get_authorized_reddit_api(config).await?;

    // post all new posts
    for post in &all_posts
    {
        if post.discussion_link.contains("https://www.reddit.com/r/")
        {
            let info = reddit_api.get_post_info(&post.discussion_link).await?;
            let fullname = &info[0].data.children[0].data.name;

            reddit_api
                .submit_crosspost(
                    &config.subreddit_name,
                    &format!("{}", &post.velocity),
                    &fullname,
                )
                .await?;
        }
        else
        {
            reddit_api
                .submit_link(
                    &config.subreddit_name,
                    &format!("{}: {} | {}", &post.source, &post.title, &post.velocity),
                    "",
                    &post.discussion_link,
                )
                .await?;
        }
    }

    info!("Done. Posted {} posts.", all_posts.len());

    // write last posted id to file
    {
        let mut last_posted_ids_file = File::create(config.get_last_posted_id_filepath())?;
        let id = format!("{}", &all_posts[0].id);
        last_posted_ids_file.write_all(id.as_bytes())?;
        info!("Recorded last posted id");
    }

    Ok(())
}

async fn delete_all_posts(config: &Config) -> Result<(), Box<dyn Error>>
{
    // create new reddit api and authorize it
    let reddit_api = get_authorized_reddit_api(config).await?;

    // get all user posts
    let user_posts = reddit_api.get_user_posts(&config.username).await?;

    // delete 'em
    for p in &user_posts.data.children
    {
        reddit_api.delete_post(&p.data.name).await?;
        info!("Deleted {:?}", &p.data.name);
    }

    info!("Done. Deleted {} posts.", user_posts.data.children.len());

    Ok(())
}
