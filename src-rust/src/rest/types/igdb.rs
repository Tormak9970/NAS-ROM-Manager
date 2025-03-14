use phf::phf_map;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBMetadataPlatform {
  pub igdbId: u64,
  pub name: String,
  pub abbreviation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBWebsite {
  pub url: String,
  pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBAgeRating {
  pub rating: String,
  pub category: String,
  pub ratingCoverUrl: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBRelatedGame {
  pub id: u64,
  pub name: String,
  pub slug: String,
  pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBMetadata {
  pub totalRating: String,
  pub aggregatedRating: String,
  pub firstReleaseDate: u64,
  pub genres: Vec<String>,
  pub franchises: Vec<String>,
  pub keywords: Vec<String>,
  pub developers: Vec<String>,
  pub publishers: Vec<String>,
  pub gameModes: Vec<String>,
  pub languages: Vec<String>,
  pub ageRatings: Vec<IGDBAgeRating>,
  pub platforms: Vec<IGDBMetadataPlatform>,
  pub dlcs: Vec<IGDBRelatedGame>,
  pub expansions: Vec<IGDBRelatedGame>,
  pub websites: Vec<IGDBWebsite>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBRom {
  pub igdbId: u64,
  pub slug: Option<String>,
  pub name: Option<String>,
  pub summary: Option<String>,
  pub coverUrl: Option<String>,
  pub thumbUrl: Option<String>,
  pub metadata: Option<IGDBMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBSearchResult {
  pub igdbId: u64,
  pub name: String,
}

pub const GAMES_FIELDS: [&'static str; 29] = [
  "id",
  "name",
  "slug",
  "summary",
  "keywords.name",
  "total_rating",
  "aggregated_rating",
  "first_release_date",
  "language_supports.language.native_name",
  "cover.url",
  "platforms.id",
  "platforms.abbreviation",
  "platforms.name",
  "genres.name",
  "franchise.name",
  "franchises.name",
  "game_modes.name",
  "involved_companies.company.name",
  "involved_companies.developer",
  "involved_companies.publisher",
  "dlcs.id",
  "dlcs.slug",
  "dlcs.name",
  "expansions.id",
  "expansions.slug",
  "expansions.name",
  "age_ratings.rating_category",
  "websites.category",
  "websites.url"
];

pub const SEARCH_FIELDS: [&'static str; 2] = ["game.id", "name"];


#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBAgeRatingStatic<'a> {
  pub rating: &'a str,
  pub category: &'a str,
  pub rating_cover_url: &'a str,
}

impl Into<IGDBAgeRating> for IGDBAgeRatingStatic<'_> {
  fn into(self) -> IGDBAgeRating {
    return IGDBAgeRating {
      rating: self.rating.to_string(),
      category: self.category.to_string(),
      ratingCoverUrl: self.rating_cover_url.to_string(),
    }
  }
}

pub static IGDB_AGE_RATINGS: phf::Map<&'static str, IGDBAgeRatingStatic<'static>> = phf_map! {
  "1" => IGDBAgeRatingStatic {
    rating: "Three",
    category: "PEGI",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/pegi/pegi_3.png",
  },
  "2" => IGDBAgeRatingStatic {
    rating: "Seven",
    category: "PEGI",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/pegi/pegi_7.png",
  },
  "3" => IGDBAgeRatingStatic {
    rating: "Twelve",
    category: "PEGI",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/pegi/pegi_12.png",
  },
  "4" => IGDBAgeRatingStatic {
    rating: "Sixteen",
    category: "PEGI",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/pegi/pegi_16.png",
  },
  "5" => IGDBAgeRatingStatic {
    rating: "Eighteen",
    category: "PEGI",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/pegi/pegi_18.png",
  },
  "6" => IGDBAgeRatingStatic {
    rating: "RP",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_rp.png",
  },
  "7" => IGDBAgeRatingStatic {
    rating: "EC",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_ec.png",
  },
  "8" => IGDBAgeRatingStatic {
    rating: "E",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_e.png",
  },
  "9" => IGDBAgeRatingStatic {
    rating: "E10",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_e10.png",
  },
  "10" => IGDBAgeRatingStatic {
    rating: "T",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_t.png",
  },
  "11" => IGDBAgeRatingStatic {
    rating: "M",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_m.png",
  },
  "12" => IGDBAgeRatingStatic {
    rating: "AO",
    category: "ESRB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/esrb/esrb_ao.png",
  },
  "13" => IGDBAgeRatingStatic {
    rating: "CERO_A",
    category: "CERO",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/cero/cero_a.png",
  },
  "14" => IGDBAgeRatingStatic {
    rating: "CERO_B",
    category: "CERO",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/cero/cero_b.png",
  },
  "15" => IGDBAgeRatingStatic {
    rating: "CERO_C",
    category: "CERO",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/cero/cero_c.png",
  },
  "16" => IGDBAgeRatingStatic {
    rating: "CERO_D",
    category: "CERO",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/cero/cero_d.png",
  },
  "17" => IGDBAgeRatingStatic {
    rating: "CERO_Z",
    category: "CERO",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/cero/cero_z.png",
  },
  "18" => IGDBAgeRatingStatic {
    rating: "USK_0",
    category: "USK",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/usk/usk_0.png",
  },
  "19" => IGDBAgeRatingStatic {
    rating: "USK_6",
    category: "USK",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/usk/usk_6.png",
  },
  "20" => IGDBAgeRatingStatic {
    rating: "USK_12",
    category: "USK",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/usk/usk_12.png",
  },
  "21" => IGDBAgeRatingStatic {
    rating: "USK_16",
    category: "USK",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/usk/usk_16.png",
  },
  "22" => IGDBAgeRatingStatic {
    rating: "USK_18",
    category: "USK",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/usk/usk_18.png",
  },
  "23" => IGDBAgeRatingStatic {
    rating: "GRAC_ALL",
    category: "GRAC",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/grac/grac_all.png",
  },
  "24" => IGDBAgeRatingStatic {
    rating: "GRAC_Twelve",
    category: "GRAC",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/grac/grac_12.png",
  },
  "25" => IGDBAgeRatingStatic {
    rating: "GRAC_Fifteen",
    category: "GRAC",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/grac/grac_15.png",
  },
  "26" => IGDBAgeRatingStatic {
    rating: "GRAC_Eighteen",
    category: "GRAC",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/grac/grac_18.png",
  },
  "27" => IGDBAgeRatingStatic {
    rating: "GRAC_TESTING",
    category: "GRAC",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/grac/grac_testing.png",
  },
  "28" => IGDBAgeRatingStatic {
    rating: "CLASS_IND_L",
    category: "CLASS_IND",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_l.png",
  },
  "29" => IGDBAgeRatingStatic {
    rating: "CLASS_IND_Ten",
    category: "CLASS_IND",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_10.png",
  },
  "30" => IGDBAgeRatingStatic {
    rating: "CLASS_IND_Twelve",
    category: "CLASS_IND",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_12.png",
  },
  "31" => IGDBAgeRatingStatic {
    rating: "ACB_G",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_g.png",
  },
  "32" => IGDBAgeRatingStatic {
    rating: "ACB_PG",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_pg.png",
  },
  "33" => IGDBAgeRatingStatic {
    rating: "ACB_M",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_m.png",
  },
  "34" => IGDBAgeRatingStatic {
    rating: "ACB_MA15",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_ma15.png",
  },
  "35" => IGDBAgeRatingStatic {
    rating: "ACB_R18",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_r18.png",
  },
  "36" => IGDBAgeRatingStatic {
    rating: "ACB_RC",
    category: "ACB",
    rating_cover_url: "https://www.igdb.com/icons/rating_icons/acb/acb_rc.png",
  },
};

pub static IGDB_WEBSITE_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
  "1" => "official",
  "2" => "wikia",
  "3" => "wikipedia",
  "4" => "facebook",
  "5" => "twitter",
  "6" => "twitch",
  "8" => "instagram",
  "9" => "youtube",
  "10" => "iphone",
  "11" => "ipad",
  "12" => "android",
  "13" => "steam",
  "14" => "reddit",
  "15" => "itch",
  "16" => "epicgames",
  "17" => "gog",
  "18" => "discord",
  "19" => "bluesky",
};



// ! Response types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBLogoResponse {
  pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBNamedResponse {
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBCoverResponse {
  pub url: Option<String>,
}

// ? Age Ratings

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBAgeRatingResponse {
  pub rating_category: u64,
}

// ? Roms
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBInvolvedCompanyResponse {
  pub company: IGDBNamedResponse,
  pub publisher: bool,
  pub developer: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBNativeLanguageResponse {
  pub native_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBLanguageResponse {
  pub language: IGDBNativeLanguageResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRelatedGameResponse {
  pub id: u64,
  pub name: Option<String>,
  pub slug: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBWebsiteResponse {
  pub category: u64,
  pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRomResponse {
  pub id: u64,
  pub slug: Option<String>,
  pub name: Option<String>,
  pub keywords: Option<Vec<IGDBNamedResponse>>,
  pub cover: Option<IGDBCoverResponse>,
  pub summary: Option<String>,
  pub total_rating: Option<f32>,
  pub aggregated_rating: Option<f32>,
  pub first_release_date: Option<u64>,
  pub genres: Vec<IGDBNamedResponse>,
  pub franchise: Option<IGDBNamedResponse>,
  pub franchises: Option<Vec<IGDBNamedResponse>>,
  pub alternative_names: Option<Vec<IGDBNamedResponse>>,
  pub involved_companies: Option<Vec<IGDBInvolvedCompanyResponse>>,
  pub game_modes: Option<Vec<IGDBNamedResponse>>,
  pub language_supports: Option<Vec<IGDBLanguageResponse>>,
  pub platforms: Option<Vec<Map<String, Value>>>,
  pub age_ratings: Option<Vec<IGDBAgeRatingResponse>>,
  pub expansions: Option<Vec<IGDBRelatedGameResponse>>,
  pub dlcs: Option<Vec<IGDBRelatedGameResponse>>,
  pub websites: Option<Vec<IGDBWebsiteResponse>>,
}

pub type IGDBRomsResponse = Vec<IGDBRomResponse>;


// ? Search
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBSearchGameResponse {
  pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBSearchResponse {
  pub name: String,
  pub game: IGDBSearchGameResponse,
}