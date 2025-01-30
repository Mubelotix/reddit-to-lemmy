#![recursion_limit = "512"]

use actix_web::{guard::{Guard, GuardContext}, web, App, HttpRequest, HttpResponse, HttpServer};
use base64::Engine;
use lemmy_client::lemmy_api_common::lemmy_db_schema::{newtypes::DbUrl, source::{community::Community, post::Post}, CommentSortType, SortType};
use log::{info, warn};
use serde::{Deserialize, Deserializer};

mod login;
mod session;
mod loid;
mod w3_reporting_policy;
mod v2c;
mod v2p;

mod ads;

mod expose_experiments;
mod get_account;
mod get_awarding_totals;
mod get_awards_for_sub;
mod get_badges;
mod get_blocked_users;
mod get_communities;
mod get_community;
mod get_custom_emojis;
mod get_dev_metadata;
mod get_dynamic_configs;
mod get_earned_gold;
mod get_email_permission;
mod get_experiments;
mod get_home_feed;
mod get_inventory_items;
mod get_location;
mod get_marketing_nudges;
mod get_matrix_notifications;
mod get_posts;
mod get_preferences;
mod get_profile;
mod get_public_showcase;
mod get_recommendations;
mod get_subscribed_count;
mod get_trophies;
mod get_username;
mod get_vaults;
mod register_mobile_push_token;
mod search_message_reactions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlRequest<V> {
    operation_name: String,
    variables: V,
    extensions: serde_json::Value,
}

pub trait HackTraitPerson {
    fn reddit_id(&self) -> String;
    fn prefixed_name(&self) -> String;
    fn formatted_name(&self) -> String;
    fn path(&self) -> String;
}

impl HackTraitPerson for lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person {
    fn reddit_id(&self) -> String {
        format!("t2_{}", self.id.0)
    }

    fn prefixed_name(&self) -> String {
        format!("u/{}", self.name)
    }

    fn formatted_name(&self) -> String {
        format!("{}@{}", self.name, "todo") // TODO
    }

    fn path(&self) -> String {
        format!("/u/{}@{}", self.name, "todo") // TODO
    }
}

pub trait HackTraitCommunity {
    fn reddit_id(&self) -> String;
    fn prefixed_name(&self) -> String;
    fn link(&self) -> String;
    fn path(&self) -> String;
}

impl HackTraitCommunity for Community {
    fn reddit_id(&self) -> String {
        format!("t5_{}", self.id.0)
    }

    fn prefixed_name(&self) -> String {
        format!("c/{}", self.name)
    }

    fn link(&self) -> String {
        format!("{}@{}", self.name, "todo") // TODO
    }

    fn path(&self) -> String {
        format!("/c/{}@{}", self.name, "todo") // TODO
    }
}

fn markdown_to_text(markdown: &str) -> String {
    fn text_content(node: markdown::mdast::Node) -> String {
        use markdown::mdast::Node::*;
    
        match node {
            Root(root) => root.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Blockquote(blockquote) => blockquote.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            FootnoteDefinition(footnote_definition) => footnote_definition.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            List(list) => list.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Delete(delete) => format!("-{}-", delete.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" ")),
            Emphasis(emphasis) => emphasis.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Link(link) => link.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Strong(strong) => strong.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Text(text) => text.value,
            Heading(heading) => heading.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            ListItem(list_item) => list_item.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            Paragraph(paragraph) => paragraph.children.into_iter().map(|c| text_content(c)).collect::<Vec<_>>().join(" "),
            _ => String::new()
        }
    }

    let ast = match markdown::to_mdast(markdown, &markdown::ParseOptions::default()) {
        Ok(ast) => ast,
        Err(e) => return format!("Invalid markdown: {e}"),
    };
    
    text_content(ast)
}

pub trait HackTraitPost {
    fn reddit_id(&self) -> String;
    fn reddit_id_base64(&self) -> String;
    fn canonical_url(&self) -> String;
}

impl HackTraitPost for Post {
    fn reddit_id(&self) -> String {
        format!("t3_{}", self.id)
    }

    fn reddit_id_base64(&self) -> String {
        base64::prelude::BASE64_STANDARD.encode(self.reddit_id())
    }

    fn canonical_url(&self) -> String {
        format!("https://jlai.lu/post/{}", self.id)
    }
}

pub trait HackTraitMediaSource {
    fn as_media_source(&self) -> Option<serde_json::Value>;
}

impl HackTraitMediaSource for Option<DbUrl> {
    fn as_media_source(&self) -> Option<serde_json::Value> {
        self.as_ref().map(|url| serde_json::json! {{
            "__typename": "MediaSource",
            "url": url,
            "dimensions": { "width": 256, "height": 256 }
        }})
    }
}

pub trait HackTraitCommentSortType {
    fn to_reddit(&self) -> &'static str;
}

impl HackTraitCommentSortType for CommentSortType {
    fn to_reddit(&self) -> &'static str {
        match self {
            CommentSortType::Hot => "CONFIDENCE",
            CommentSortType::Top => "TOP",
            CommentSortType::New => "NEW",
            CommentSortType::Old => "OLD",
            CommentSortType::Controversial => "CONTROVERSIAL",
        }
    }
}

pub trait HackTraitSortType: Sized {
    fn deserialize_from_reddit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

impl HackTraitSortType for Option<SortType> {
    fn deserialize_from_reddit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let sort_type = String::deserialize(deserializer)?;
        match sort_type.as_str() {
            "BEST" => Ok(Some(SortType::Active)),
            "NEW" => Ok(Some(SortType::New)),
            "HOT" => Ok(Some(SortType::Hot)),
            "TOP" => Ok(Some(SortType::TopDay)),
            "CONTROVERSIAL" => Ok(Some(SortType::Controversial)),
            "RISING" => Ok(Some(SortType::Scaled)),
            "NONE" => Ok(None),
            _ => Err(serde::de::Error::custom(format!("Unknown sort type: {sort_type}"))),
        }
    }
}

pub fn get_jwt(req: &HttpRequest) -> Option<String> {
    let autorization = req.headers().get("authorization")?.to_str().ok()?;
    let jwt = autorization.split_once(' ')?.1.to_owned();
    Some(jwt)
}

struct Apollo(&'static str);

impl Guard for Apollo {
    fn check(&self, req: &GuardContext) -> bool {
        req.head().headers().get("x-apollo-operation-name").map(|o| o == self.0).unwrap_or(false)
    }
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = std::env::var("PORT").map(|p| p.parse().expect("Port must be a number")).unwrap_or(3000);

    let fut = HttpServer::new(|| {
        App::new()
            .service(login::login)
            .service(session::session)
            .service(w3_reporting_policy::w3_reporting_policy)
            .service(loid::loid)
            .service(v2c::v2c)
            .service(v2p::v2p)
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("AdEligibilityForUser")).to(ads::get_ad_eligibility))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("AllDynamicConfigs")).to(get_dynamic_configs::get_dynamic_configs))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("AwardingTotalsForPost")).to(get_awarding_totals::get_awarding_totals))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("BadgeCount")).to(get_badges::get_badges))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("BlockedRedditors")).to(get_blocked_users::get_blocked_users))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("CommentTreeAds")).to(ads::get_comment_tree_ads))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("CommentsPageAdPost")).to(ads::get_comments_page_ad))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("DiscoverBarRecommendations")).to(get_recommendations::get_recommendations))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("EmailPermission")).to(get_email_permission::get_email_permission))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("ExposeExperiments")).to(expose_experiments::expose_experiments))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAccount")).to(get_account::get_account))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAccountPreferences")).to(get_preferences::get_preferences))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAllVaults")).to(get_vaults::get_vaults))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetAwardsForSubreddit")).to(get_awards_for_sub::get_awards_for_sub))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetCustomEmojisStatus")).to(get_custom_emojis::get_custom_emojis))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetDevPlatformMetadata")).to(get_dev_metadata::get_dev_metadata))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetEarnedGoldBalance")).to(get_earned_gold::get_earned_gold_balance))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetInventoryItemsByIds")).to(get_inventory_items::get_inventory_items))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetPublicShowcaseOfCurrentUser")).to(get_public_showcase::get_public_showcase))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("GetRealUsername")).to(get_username::get_username))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("HomeFeedSdui")).to(get_home_feed::get_home_feed))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("IdentityMatrixNotifications")).to(get_matrix_notifications::get_matrix_notifications))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("MarketingNudges")).to(get_marketing_nudges::get_marketing_nudges))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("PostsByIds")).to(get_posts::get_posts))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("ProfileTrophies")).to(get_trophies::get_trophies))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("RegisterMobilePushToken")).to(register_mobile_push_token::register_mobile_push_token))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("SearchChatMessageReactionIcons")).to(search_message_reactions::search_message_reactions))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("SubredditStructuredStyle")).to(get_community::get_community))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("SubscribedSubredditsCount")).to(get_subscribed_count::get_subscribed_count))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UserLocation")).to(get_location::get_location))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UsernameAndExperiments")).to(get_experiments::get_experiments))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UserProfile")).to(get_profile::get_profile))
            .route("/gql-fed.reddit.com/", web::post().guard(Apollo("UserSubredditListItems")).to(get_communities::get_communities))
            .route("/gql-fed.reddit.com/", web::to(|req: HttpRequest| async move {
                    let operation_name = req.headers().get("x-apollo-operation-name").map(|o| o.to_str().unwrap()).unwrap_or("unknown");
                    warn!("Unknown Apollo operation: {operation_name}");
                    HttpResponse::InternalServerError().finish()
            }))
            .default_service(web::to(|req: HttpRequest| async move {
                warn!("Unhandled request: {req:?}");
                HttpResponse::InternalServerError().finish()
            }))
    })
    .bind(("127.0.0.1", port))?
    .run();

    info!("Server running at http://localhost:{port}");

    fut.await
}
