// use async_recursion::async_recursion;
// use futures::stream::{repeat, Repeat};
use log::debug;
use reqwest::Client;
use reqwest::RequestBuilder;
use reqwest::Response;
use std::{collections::HashMap, error::Error};
use url::Url;

use super::reddit_models::*;

const REDDIT_AUTH_HOST: &str = "https://www.reddit.com/api";
const REDDIT_API_HOST: &str = "https://oauth.reddit.com";
static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn get_auth_url(path: &str) -> Result<String, Box<dyn Error>>
{
    let url = Url::parse(&format!("{}/v1{}", REDDIT_AUTH_HOST, path))?;
    debug!("{}", &url);
    Ok(url.to_string())
}

fn get_api_url(path: &str) -> Result<String, Box<dyn Error>>
{
    let url = Url::parse(&format!("{}{}", REDDIT_API_HOST, path))?;
    debug!("{}", &url);
    Ok(url.to_string())
}

pub struct RedditApi
{
    client: Client,
    auth:   Option<AuthResponse>,
}

impl RedditApi
{
    pub fn new() -> Self
    {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .ok()
            .expect("reqwest client to be built");

        Self { client, auth: None }
    }

    async fn send_request(&self, request: RequestBuilder) -> Result<Response, Box<dyn Error>>
    {
        let mut req = request;

        // use auth if exists
        if let Some(auth) = &self.auth
        {
            req = req.bearer_auth(&auth.access_token);
        }

        Ok(req.send().await?)
    }

    pub async fn authorize(
        &mut self,
        username: &str,
        password: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let url = &get_auth_url("/access_token")?;

        let request = self
            .client
            .post(url)
            .basic_auth(client_id, Some(client_secret))
            .query(&[
                ("grant_type", "password"),
                ("username", username),
                ("password", password),
            ]);

        let auth_response = self
            .send_request(request)
            .await?
            .json::<AuthResponse>()
            .await?;

        // set the auth
        self.auth = Some(auth_response);

        Ok(())
    }

    // Reddit docs: https://bit.ly/2Geh0Bh
    pub async fn submit_link(
        &self,
        subreddit: &str,
        title: &str,
        text: &str,
        link: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let url = &get_api_url("/api/submit")?;

        let mut hashmap = HashMap::new();
        hashmap.insert("sr", subreddit);
        hashmap.insert("title", title);
        hashmap.insert("text", text);
        hashmap.insert("url", link);
        hashmap.insert("kind", "link");

        let request = self
            .client
            .post(url)
            .query(&hashmap)
            .query(&[("resubmit", true)]);

        let response = self.send_request(request).await?.text().await?;

        debug!("{}", response);

        // Ok(response)
        Ok(())
    }

    // Reddit docs: https://bit.ly/2Geh0Bh
    pub async fn submit_crosspost(
        &self,
        subreddit: &str,
        title: &str,
        crosspost_fullname: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let url = &get_api_url("/api/submit")?;

        // how to submit a crosspost https://bit.ly/3i0FICt
        let mut hashmap = HashMap::new();
        hashmap.insert("sr", subreddit);
        hashmap.insert("title", title);
        hashmap.insert("crosspost_fullname", crosspost_fullname);
        hashmap.insert("kind", "crosspost");

        let request = self
            .client
            .post(url)
            .query(&hashmap)
            .query(&[("resubmit", true)]);

        let response = self.send_request(request).await?.text().await?;

        debug!("{}", response);

        // Ok(response)
        Ok(())
    }

    pub async fn get_post_info(
        &self,
        post_link: &str,
    ) -> Result<Vec<ListingResponse>, Box<dyn Error>>
    {
        let url = format!("{}.json", post_link);
        debug!("{}", &url);
        let request = self.client.get(&url);
        let response_text = request.send().await?.text().await?;
        let response = serde_json::from_str(&response_text)?;

        Ok(response)
    }

    pub async fn get_user_posts(&self, username: &str) -> Result<ListingResponse, Box<dyn Error>>
    {
        let url = &get_api_url(&format!("/user/{}/submitted", username))?;

        let request = self.client.get(url).query(&[("limit", 100)]);

        let response = self
            .send_request(request)
            .await?
            .json::<ListingResponse>()
            .await?;

        Ok(response)
    }

    pub async fn delete_post(&self, post_fullname: &str) -> Result<(), Box<dyn Error>>
    {
        let url = &get_api_url("/api/del")?;

        let request = self.client.post(url).query(&[("id", post_fullname)]);
        self.send_request(request).await?;

        Ok(())
    }
}
