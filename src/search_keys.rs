
pub trait SearchKey {
    fn to_string(&self) -> String;
}

// impl std::fmt::Display for dyn SearchKey {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "search_key={}", self)
//     }
// }

macro_rules! enum_str {
    (
        enum $name:ident {
            $($variant:ident => $str:expr),* $(,)?
        }
    ) => {
        pub enum $name {
            $($variant),*
        }

        impl SearchKey for $name {
            fn to_string(&self) -> String {
                match self {
                    $($name::$variant => $str.to_string()),*
                }
            }
        }
    }
}


enum_str!{
    enum RealEstate {
        Homes           => "SEARCH_ID_REALESTATE_HOMES",
        Lettings        => "SEARCH_ID_REALESTATE_LETTINGS",
        NewBuildings    => "SEARCH_ID_REALESTATE_NEWBUILDINGS",
        Plots           => "SEARCH_ID_REALESTATE_PLOTS",
        Leisure         => "SEARCH_ID_REALESTATE_LEISURE_SALE",
        LeisurePlots    => "SEARCH_ID_REALESTATE_LEISURE_PLOTS",
        Commercial      => "SEARCH_ID_COMMERCIAL_SALE",
        CommercialRent  => "SEARCH_ID_COMMERCIAL_RENT",
        CommercialPlots => "SEARCH_ID_COMMERCIAL_PLOTS",
        Company         => "SEARCH_ID_COMPANY_SALE",
    }
}

enum_str! {
    enum MC{
        Used       => "SEARCH_ID_MC_USED",
        Scooter    => "SEARCH_ID_MC_SCOOTER",
        Snowmobile => "SEARCH_ID_MC_SNOWMOBILE",
        Atv        => "SEARCH_ID_MC_ATV",
    }
}

enum_str! {
    enum Car{
        Used       => "SEARCH_ID_CAR_USED",
        Imports    => "SEARCH_ID_CAR_PARALLEL_IMPORT",
        MobileHome => "SEARCH_ID_CAR_MOBILE_HOME",
        Caravan    => "SEARCH_ID_CAR_CARAVAN",
        Van        => "SEARCH_ID_CAR_VAN",
    }
}

enum_str! {
    enum B2B {
        Truck           => "SEARCH_ID_CAR_TRUCK",
        Bus             => "SEARCH_ID_CAR_BUS",
        Agriculture     => "SEARCH_ID_CAR_AGRI",
        Tractor         => "SEARCH_ID_AGRICULTURE_TRACTOR",
        Threshing       => "SEARCH_ID_AGRICULTURE_THRESHING",
        AgricultureTool => "SEARCH_ID_AGRICULTURE_TOOL",
    }
}

enum_str! {
    enum Boat {
        Used             => "SEARCH_ID_BOAT_USED",
        UsedWanted       => "SEARCH_ID_BOAT_USED_WANTED",
        Rent             => "SEARCH_ID_BOAT_RENT",
        Motor            => "SEARCH_ID_BOAT_MOTOR",
        MotorParts       => "SEARCH_ID_BOAT_MOTOR_PARTS",
        MotorPartsWanted => "SEARCH_ID_BOAT_PARTS_MOTOR_WANTED",
        Dock             => "SEARCH_ID_BOAT_DOCK",
        DockWanted       => "SEARCH_ID_BOAT_DOCK_WANTED",
    }
}
enum_str! {
    enum Bap {
        Common => "SEARCH_ID_BAP_COMMON"
    }    
}