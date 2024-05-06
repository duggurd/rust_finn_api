pub trait Param {
    fn to_string(&self) -> String;
}


/// Finn API Parameters
/// Some params are only available for certain search keys
/// How can i validate this? --> Metadata with each request contains valid params, run query for each param
/// could even integration test this :)
pub enum CommonParam {
    Query(String),
    PageNum(i32),
    PriceFrom(i32),
    PriceTo(i32),
    PublishedToday(bool),
    Condition(Condition),
    DealerSegment(DealerSegment),
    TradeType(TradeType),
    Area,
    ShippingExists(bool),
}

impl Param for CommonParam {
    fn to_string(&self) -> String {
        match self {
            CommonParam::Query(value) => format!("q={}", value),
            CommonParam::PageNum(value) => format!("page={}", value),
            CommonParam::PriceFrom(value) => format!("price_from={}", value),
            CommonParam::PriceTo(value) => format!("price_to={}", value),
            CommonParam::PublishedToday(value) => {
                format!("published={}", match value {
                    true => "1",
                    false => ""
                })
            },
            CommonParam::Condition(value) => value.to_string(),
            CommonParam::DealerSegment(value) => value.to_string(),
            CommonParam::TradeType(value ) => value.to_string(),
            CommonParam::Area => todo!(),
            CommonParam::ShippingExists(value) => {
                format!("shipping_exists={}", match value {
                    true => "1",
                    false => ""
                })
            }
        }
    }

}

pub enum REParam {}
pub enum CarParam {}


/// Trade types
pub enum TradeType {
    ForSale,
    Free,
    Wanted,
    All
}

impl Param for TradeType {
    fn to_string(&self) -> String {
        let value = match self {
            TradeType::ForSale => "1",
            TradeType::Free => "2",
            TradeType::Wanted => "3",
            TradeType::All => "",
        };
        
        format!("trade_type={}", value)
    }
}



/// Dealer segments
pub enum DealerSegment {
    Private,
    Dealer,
    All
}

impl Param for DealerSegment {
    fn to_string(&self) -> String {
        let value = match self {
            DealerSegment::Private => "1",
            DealerSegment::Dealer => "3",
            DealerSegment::All => "",
        };
        
        format!("dealer_segment={}", value)
    }
}

/// Conditions
pub enum Condition {
    New,
    Used,
    All
}

impl Param for Condition {
    fn to_string(&self) -> String {
        
        let value = match self {
            Condition::New => "2",
            Condition::Used => "4",
            Condition::All => "",
        };
        
        format!("condition={}", value)
    }    
}