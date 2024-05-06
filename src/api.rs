use crate::search_keys::SearchKey;
use crate::search_keys;
use crate::params;
use crate::params::{CommonParam, Param};
use crate::Result;


use reqwest;

/// The Finn API client
struct FinnApi {
    client: reqwest::Client,
}

/// Finn API request builder
struct ApiRequest {
    search_key: String,
    params: Vec<String>
}


impl FinnApi {

    async fn new(client: Option<reqwest::Client>) -> Self {
        match client {
            Some(client) => Self {client},
            None => Self {client: reqwest::Client::new()}
        }
    }

    async fn get(api_request: ApiRequest) -> Result<reqwest::Response> {
        let url = api_request.build()?;
        let resp = reqwest::get(url).await?;
        Ok(resp)
    }
}


impl ApiRequest {
    const API_ENDPOINT: &'static str = "https://www.finn.no/api";

    fn new() -> Self {
        Self {
            search_key: String::new(),
            params: Vec::new()
        }
    }
    fn search_key(mut self, search_key: impl SearchKey) -> Self {
        self.search_key = search_key.to_string();
        
        self
    }

    fn param(mut self, param: impl Param) -> Self{
        self.params.push(param.to_string());

        self
    }

    fn build(self) -> Result<String> {
        match self.search_key.is_empty() {
            true => Err("No search key provided".into()),
            
            false =>  {
                let params = self.params
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join("&");
                
                Ok(format!("{}?search_key={}&{}", ApiRequest::API_ENDPOINT, self.search_key, params))
            
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_api_request() {
        use super::*;
        let api_request = ApiRequest::new()
            .search_key(search_keys::RealEstate::Homes)
            .param(CommonParam::Query("oslo".to_string()))
            .param(CommonParam::PageNum(1))
            .param(CommonParam::PriceFrom(1000000))
            .param(CommonParam::PriceTo(5000000))
            .param(CommonParam::PublishedToday(false))
            .param(CommonParam::Condition(params::Condition::New))
            .param(CommonParam::DealerSegment(params::DealerSegment::Private))
            .param(CommonParam::TradeType(params::TradeType::ForSale))
            .param(CommonParam::ShippingExists(true));

        let url = api_request.build().unwrap();
        assert_eq!(url, "https://www.finn.no/api?search_key=SEARCH_ID_REALESTATE_HOMES&q=oslo&page=1&price_from=1000000&price_to=5000000&published=&condition=2&dealer_segment=1&trade_type=1&shipping_exists=1");
    }

    #[test]
    fn test_only_sk() {
        use super::*;
        let api_request = ApiRequest::new()
            .search_key(search_keys::RealEstate::Homes);

        let url = api_request.build().unwrap();
        assert_eq!(url, "https://www.finn.no/api?search_key=SEARCH_ID_REALESTATE_HOMES&");
    }

    #[test]
    fn test_no_sk() {
        use super::*;
        let api_request = ApiRequest::new()
            .param(CommonParam::Query("oslo".to_string()))
            .param(CommonParam::PageNum(1))
            .param(CommonParam::PriceFrom(1000000))
            .param(CommonParam::PriceTo(5000000))
            .param(CommonParam::PublishedToday(false))
            .param(CommonParam::Condition(params::Condition::New))
            .param(CommonParam::DealerSegment(params::DealerSegment::Private))
            .param(CommonParam::TradeType(params::TradeType::ForSale))
            .param(CommonParam::ShippingExists(true));

        let url = api_request.build();
        assert!(url.is_err());
    }

    #[test]
    fn test_all() {
        use super::*;
        let api_request = ApiRequest::new()
            .search_key(search_keys::RealEstate::Homes)
            .param(CommonParam::Condition(params::Condition::All))
            .param(CommonParam::DealerSegment(params::DealerSegment::All))
            .param(CommonParam::TradeType(params::TradeType::All));

        let url = api_request.build().unwrap();
        assert_eq!(url, "https://www.finn.no/api?search_key=SEARCH_ID_REALESTATE_HOMES&condition=&dealer_segment=&trade_type=");
    }
   
   
}