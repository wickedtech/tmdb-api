use crate::people::PersonShort;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditCommon {
    pub credit_id: String,
    pub adult: bool,
    pub known_for_department: Option<String>,
    pub original_name: String,
    pub popularity: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cast {
    #[serde(flatten)]
    pub credit: CreditCommon,
    #[serde(flatten)]
    pub person: PersonShort,
    pub cast_id: Option<u64>,
    pub character: String,
    pub order: u64,
    #[serde(default)]
    pub profile_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Crew {
    #[serde(flatten)]
    pub credit: CreditCommon,
    #[serde(flatten)]
    pub person: PersonShort,
    pub department: String,
    pub job: String,
    #[serde(default)]
    pub profile_path: Option<String>,
}
