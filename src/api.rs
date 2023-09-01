use serde::Deserialize;

const BORED_API_URL: &str = "http://www.boredapi.com/api/activity/";

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
    pub async fn get_random() -> Result<Self, reqwest::Error> {
        let activity = reqwest::get(BORED_API_URL).await?.json::<Self>().await?;
        Ok(activity)
    }

    pub fn get_pretty(&self) -> String {
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
