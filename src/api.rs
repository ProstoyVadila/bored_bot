use serde::Deserialize;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

const BORED_API_URL: &str = "https://www.boredapi.com/api/activity";

#[derive(Debug, EnumIter, Display, EnumString)]
pub enum ActivityType {
    Education,
    Recreational,
    Social,
    Diy,
    Charity,
    Cooking,
    Relaxation,
    Music,
    Busywork,
}

impl ActivityType {
    pub fn get_all(&self) -> String {
        ActivityType::iter()
            .map(|activity_type| activity_type.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Deserialize, Debug)]
pub struct BoredActivity {
    activity: String,
    #[serde(rename = "type")]
    activity_type: String,
    participants: u8,
    price: f32,
    link: Option<String>,
    accessibility: f32,
    // key: String,
}

impl BoredActivity {
    pub fn get_pretty_msg(&self) -> String {
        format!(
            "Activity: {}\nType: {}\nParticipants: {}\nPrice: {}\nAccessibility: {}\nLink: {}",
            self.activity,
            self.activity_type,
            self.participants,
            self.price,
            self.accessibility,
            self.link.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

async fn do_request(url: &str) -> Result<BoredActivity, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    debug!("Bored API response status: {:?}", resp.status());
    let activity = resp.json::<BoredActivity>().await?;
    Ok(activity)
}
pub async fn get_random() -> Result<BoredActivity, reqwest::Error> {
    do_request(BORED_API_URL).await
}

pub async fn get_by_type(activity_type: ActivityType) -> Result<BoredActivity, reqwest::Error> {
    let url = format!(
        "{}?type={}",
        BORED_API_URL,
        activity_type.to_string().to_lowercase()
    );
    do_request(&url).await
}

pub async fn get_by_participants(participants: u8) -> Result<BoredActivity, reqwest::Error> {
    let url = format!("{}?participants={}", BORED_API_URL, participants);
    do_request(&url).await
}

pub async fn get_by_price(price: f32) -> Result<BoredActivity, reqwest::Error> {
    let url = format!("{}?price={}", BORED_API_URL, price);
    do_request(&url).await
}
