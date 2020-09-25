extern crate serde_derive;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse
{
    pub access_token: String,
    pub token_type:   String,
    pub expires_in:   u32,
    pub scope:        String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingResponse
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: ListingData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<i64>,

    #[serde(rename = "children")]
    pub children: Vec<ListingChild>,

    #[serde(rename = "after")]
    pub after: Option<String>,

    #[serde(rename = "before")]
    pub before: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: ListingChildData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingChildData
{
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "selftext")]
    pub selftext: Option<String>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "clicked")]
    pub clicked: Option<bool>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,

    #[serde(rename = "pwls")]
    pub pwls: Option<i64>,

    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: Option<String>,

    #[serde(rename = "hide_score")]
    pub hide_score: Option<bool>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "quarantine")]
    pub quarantine: Option<bool>,

    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: Option<String>,

    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: Option<f64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<String>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "media_embed")]
    pub media_embed: Option<Gildings>,

    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "is_original_content")]
    pub is_original_content: Option<bool>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "secure_media")]
    pub secure_media: Option<serde_json::Value>,

    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: Option<bool>,

    #[serde(rename = "is_meta")]
    pub is_meta: Option<bool>,

    #[serde(rename = "category")]
    pub category: Option<serde_json::Value>,

    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: Option<Gildings>,

    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<serde_json::Value>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<String>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<AuthorFlairRichtext>>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "post_hint")]
    pub post_hint: Option<String>,

    #[serde(rename = "content_categories")]
    pub content_categories: Option<serde_json::Value>,

    #[serde(rename = "is_self")]
    pub is_self: Option<bool>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "link_flair_type")]
    pub link_flair_type: Option<String>,

    #[serde(rename = "wls")]
    pub wls: Option<i64>,

    #[serde(rename = "removed_by_category")]
    pub removed_by_category: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "domain")]
    pub domain: Option<String>,

    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: Option<bool>,

    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "suggested_sort")]
    pub suggested_sort: Option<serde_json::Value>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "url_overridden_by_dest")]
    pub url_overridden_by_dest: Option<String>,

    #[serde(rename = "view_count")]
    pub view_count: Option<serde_json::Value>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: Option<bool>,

    #[serde(rename = "pinned")]
    pub pinned: Option<bool>,

    #[serde(rename = "over_18")]
    pub over_18: Option<bool>,

    #[serde(rename = "preview")]
    pub preview: Option<Preview>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<AllAwarding>>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "media_only")]
    pub media_only: Option<bool>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "spoiler")]
    pub spoiler: Option<bool>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<String>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "visited")]
    pub visited: Option<bool>,

    #[serde(rename = "removed_by")]
    pub removed_by: Option<serde_json::Value>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: Option<bool>,

    #[serde(rename = "num_duplicates")]
    pub num_duplicates: Option<i64>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "discussion_type")]
    pub discussion_type: Option<serde_json::Value>,

    #[serde(rename = "num_comments")]
    pub num_comments: Option<i64>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "media")]
    pub media: Option<serde_json::Value>,

    #[serde(rename = "contest_mode")]
    pub contest_mode: Option<bool>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<String>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "whitelist_status")]
    pub whitelist_status: Option<String>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: Option<i64>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "num_crossposts")]
    pub num_crossposts: Option<i64>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "is_video")]
    pub is_video: Option<bool>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllAwarding
{
    #[serde(rename = "giver_coin_reward")]
    pub giver_coin_reward: Option<serde_json::Value>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<serde_json::Value>,

    #[serde(rename = "is_new")]
    pub is_new: Option<bool>,

    #[serde(rename = "days_of_drip_extension")]
    pub days_of_drip_extension: Option<i64>,

    #[serde(rename = "coin_price")]
    pub coin_price: Option<i64>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "penny_donate")]
    pub penny_donate: Option<serde_json::Value>,

    #[serde(rename = "coin_reward")]
    pub coin_reward: Option<i64>,

    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,

    #[serde(rename = "days_of_premium")]
    pub days_of_premium: Option<i64>,

    #[serde(rename = "icon_height")]
    pub icon_height: Option<i64>,

    #[serde(rename = "tiers_by_required_awardings")]
    pub tiers_by_required_awardings: Option<serde_json::Value>,

    #[serde(rename = "resized_icons")]
    pub resized_icons: Option<Vec<ResizedIcon>>,

    #[serde(rename = "icon_width")]
    pub icon_width: Option<i64>,

    #[serde(rename = "static_icon_width")]
    pub static_icon_width: Option<i64>,

    #[serde(rename = "start_date")]
    pub start_date: Option<serde_json::Value>,

    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,

    #[serde(rename = "awardings_required_to_grant_benefits")]
    pub awardings_required_to_grant_benefits: Option<serde_json::Value>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "end_date")]
    pub end_date: Option<serde_json::Value>,

    #[serde(rename = "subreddit_coin_reward")]
    pub subreddit_coin_reward: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "static_icon_height")]
    pub static_icon_height: Option<i64>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "resized_static_icons")]
    pub resized_static_icons: Option<Vec<ResizedIcon>>,

    #[serde(rename = "icon_format")]
    pub icon_format: Option<serde_json::Value>,

    #[serde(rename = "award_sub_type")]
    pub award_sub_type: Option<String>,

    #[serde(rename = "penny_price")]
    pub penny_price: Option<serde_json::Value>,

    #[serde(rename = "award_type")]
    pub award_type: Option<String>,

    #[serde(rename = "static_icon_url")]
    pub static_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResizedIcon
{
    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "width")]
    pub width: Option<i64>,

    #[serde(rename = "height")]
    pub height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorFlairRichtext
{
    #[serde(rename = "e")]
    pub e: Option<String>,

    #[serde(rename = "t")]
    pub t: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gildings {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview
{
    #[serde(rename = "images")]
    pub images: Option<Vec<Image>>,

    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image
{
    #[serde(rename = "source")]
    pub source: Option<ResizedIcon>,

    #[serde(rename = "resolutions")]
    pub resolutions: Option<Vec<ResizedIcon>>,

    #[serde(rename = "variants")]
    pub variants: Option<Gildings>,

    #[serde(rename = "id")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleReplies
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<FluffyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<serde_json::Value>,

    #[serde(rename = "children")]
    pub children: Option<Vec<FluffyChild>>,

    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<TentacledData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledData
{
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "replies")]
    pub replies: Option<HilariousReplies>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyReplies
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<StickyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<serde_json::Value>,

    #[serde(rename = "children")]
    pub children: Option<Vec<TentacledChild>>,

    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<IndigoData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoData
{
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "replies")]
    pub replies: Option<AmbitiousReplies>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<AllAwarding>>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledReplies
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<IndecentData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<serde_json::Value>,

    #[serde(rename = "children")]
    pub children: Option<Vec<StickyChild>>,

    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<HilariousData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HilariousData
{
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "replies")]
    pub replies: Option<CunningReplies>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyReplies
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<AmbitiousData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbitiousData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<serde_json::Value>,

    #[serde(rename = "children")]
    pub children: Option<Vec<IndigoChild>>,

    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<CunningData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CunningData
{
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "replies")]
    pub replies: Option<MagentaReplies>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoReplies
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<MagentaData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagentaData
{
    #[serde(rename = "modhash")]
    pub modhash: Option<String>,

    #[serde(rename = "dist")]
    pub dist: Option<serde_json::Value>,

    #[serde(rename = "children")]
    pub children: Option<Vec<IndecentChild>>,

    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentChild
{
    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<FriskyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriskyData
{
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,

    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "comment_type")]
    pub comment_type: Option<serde_json::Value>,

    #[serde(rename = "awarders")]
    pub awarders: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    pub banned_by: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    pub ups: Option<i64>,

    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,

    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_id")]
    pub link_id: Option<String>,

    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    pub likes: Option<serde_json::Value>,

    #[serde(rename = "replies")]
    pub replies: Option<String>,

    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "saved")]
    pub saved: Option<bool>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    pub gilded: Option<i64>,

    #[serde(rename = "archived")]
    pub archived: Option<bool>,

    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,

    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,

    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,

    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<serde_json::Value>,

    #[serde(rename = "approved_by")]
    pub approved_by: Option<serde_json::Value>,

    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "is_submitter")]
    pub is_submitter: Option<bool>,

    #[serde(rename = "downs")]
    pub downs: Option<i64>,

    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,

    #[serde(rename = "gildings")]
    pub gildings: Option<Gildings>,

    #[serde(rename = "collapsed_reason")]
    pub collapsed_reason: Option<serde_json::Value>,

    #[serde(rename = "associated_award")]
    pub associated_award: Option<serde_json::Value>,

    #[serde(rename = "stickied")]
    pub stickied: Option<bool>,

    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,

    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,

    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "score_hidden")]
    pub score_hidden: Option<bool>,

    #[serde(rename = "permalink")]
    pub permalink: Option<String>,

    #[serde(rename = "num_reports")]
    pub num_reports: Option<serde_json::Value>,

    #[serde(rename = "locked")]
    pub locked: Option<bool>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "created")]
    pub created: Option<f64>,

    #[serde(rename = "subreddit")]
    pub subreddit: Option<String>,

    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,

    #[serde(rename = "controversiality")]
    pub controversiality: Option<i64>,

    #[serde(rename = "depth")]
    pub depth: Option<i64>,

    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "collapsed_because_crowd_control")]
    pub collapsed_because_crowd_control: Option<serde_json::Value>,

    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "mod_note")]
    pub mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    pub distinguished: Option<serde_json::Value>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "children")]
    pub children: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousReplies
{
    FluffyReplies(FluffyReplies),

    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousReplies
{
    String(String),

    TentacledReplies(TentacledReplies),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningReplies
{
    StickyReplies(StickyReplies),

    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaReplies
{
    IndigoReplies(IndigoReplies),

    String(String),
}
