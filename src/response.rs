use std::fs::Metadata;

use serde::{Deserialize, Serialize};

use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub docs: Vec<Ad>,
    pub filters: Vec<Filter>,
    pub metadata: MetaData,
    
    #[serde(rename = "mapUrl")]
    pub map_url: String,
    
    #[serde(rename = "newMapUrl")]
    pub new_map_url: String,

    #[serde(rename = "pageMetadata")]
    pub page_metadata: Value,

    #[serde(rename = "resultHeading")]
    pub result_heading: String,
    
    #[serde(rename = "promotedAd")]
    pub promoted_ad: Ad

}

// #[derive(Serialize, Deserialize)]
// struct PageMetadata {

// }


#[derive(Serialize, Deserialize)]
pub struct Ad {
    #[serde(rename = "type")]
    pub _type: String,

    pub id: String,
    pub main_search_key: String,
    pub heading: String,
    pub location: String,

    // #[serde(skip)]
    pub image: Image,
    pub flags: Vec<String>,
    pub timestamp: i64,
    pub coordinates: Coordinates,
    pub ad_type: i64,
    pub labels: Vec<Label>,
    pub canonical_url: String,
    pub extras: Vec<String>,
    pub price: Price,
    pub distance: i64,
    pub trade_type: String,
    pub image_urls: Vec<String>,
    pub ad_id: i64
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub display_name: String,
    pub name: String,
    pub filter_items: Vec<Value>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub params: Value,
    pub search_key: String,
    pub selected_filters: Value,
    pub num_results: i64,
    pub quest_time: i64,
    pub solr_time: i64,
    pub solr_elapsed_time: i64,
    pub result_size: Value,
    pub paging: Paging,
    pub title: String,
    pub is_savable_search: bool,
    pub search_key_description: String,
    pub vertical: String,
    pub vertical_description: String,
    pub sort: String,
    pub descriptions: Value,
    pub uuid: String,
    pub tracking: Value,
    pub guided_search: GuidedSearch,
    pub actions: Vec<Value>,
    pub is_end_of_paging: bool,
}


#[derive(Serialize, Deserialize)]
pub struct GuidedSearch {
    pub suggestions: Vec<Suggestion>
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub display_name: String,
    pub param: String,
    pub no_of_hits: i64,
    pub types: Vec<String>,
    pub score: f64,
    pub selected: bool,
    pub algorithm: String,
    pub image: Value,
    pub hash: String,
    pub tracking_object: Value,
}
#[derive(Serialize, Deserialize)]
pub struct Paging {
    pub param: String,
    pub current: i64,
    pub last: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Image {}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub _type: String,
}


#[derive(Serialize, Deserialize)]
pub struct Price {
    pub amount: i64,
    pub currency_code: String,
    pub price_unit: String,
}