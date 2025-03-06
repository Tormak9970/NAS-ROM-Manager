use phf::phf_map;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBMetadataPlatform {
  pub igdb_id: String,
  pub name: String,
  pub abbreviation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBAgeRating {
  pub rating: String,
  pub category: String,
  pub rating_cover_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRelatedGame {
  pub id: String,
  pub name: String,
  pub slug: String,
  pub r#type: String,
  pub cover_url: String,
  pub thumb_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBMetadata {
  pub total_rating: String,
  pub aggregated_rating: String,
  pub first_release_date: u64,
  pub genres: Vec<String>,
  pub franchises: Vec<String>,
  pub alternative_names: Vec<String>,
  pub collections: Vec<String>,
  pub companies: Vec<String>,
  pub game_modes: Vec<String>,
  pub age_ratings: Vec<IGDBAgeRating>,
  pub platforms: Vec<IGDBMetadataPlatform>,
  pub expansions: Vec<IGDBRelatedGame>,
  pub dlcs: Vec<IGDBRelatedGame>,
  pub similar_games: Vec<IGDBRelatedGame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRom {
  pub igdb_id: Option<String>,
  pub slug: Option<String>,
  pub name: Option<String>,
  pub summary: Option<String>,
  pub url_cover: Option<String>,
  pub url_thumb: Option<String>,
  pub igdb_metadata: Option<IGDBMetadata>,
}

pub const GAMES_FIELDS: [&'static str; 33] = [
  "id",
  "name",
  "slug",
  "summary",
  "total_rating",
  "aggregated_rating",
  "first_release_date",
  "artworks.url",
  "cover.url",
  "platforms.id",
  "platforms.abbreviation",
  "platforms.name",
  "alternative_names.name",
  "genres.name",
  "franchise.name",
  "franchises.name",
  "collections.name",
  "game_modes.name",
  "involved_companies.company.name",
  "expansions.id",
  "expansions.slug",
  "expansions.name",
  "expansions.cover.url",
  "dlcs.id",
  "dlcs.name",
  "dlcs.slug",
  "dlcs.cover.url",
  "similar_games.id",
  "similar_games.slug",
  "similar_games.name",
  "similar_games.cover.url",
  "age_ratings.rating",
  "videos.video_id",
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
      rating_cover_url: self.rating_cover_url.to_string(),
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

pub enum GameCategory {
  MainGame = 0,
  DlcAddon = 1,
  Expansion = 2,
  Bundle = 3,
  StandaloneExpansion = 4,
  Mod = 5,
  Episode = 6,
  Season = 7,
  Remake = 8,
  Remaster = 9,
  ExpandedGame = 10,
  Port = 11,
  Fork = 12,
  Pack = 13,
  Update = 14,
}



// ! Response types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBLogoResponse {
  pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBNamedResponse {
  pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBCoverResponse {
  pub url: Option<String>,
}


// ? Roms
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBInvolvedCompanyResponse {
  pub company: IGDBNamedResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRelatedGameResponse {
  pub id: Option<String>,
  pub name: Option<String>,
  pub slug: Option<String>,
  pub cover: IGDBCoverResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBRomResponse {
  pub id: Option<String>,
  pub slug: Option<String>,
  pub name: Option<String>,
  pub cover: Option<IGDBCoverResponse>,
  pub summary: Option<String>,
  pub total_rating: Option<String>,
  pub aggregated_rating: Option<String>,
  pub first_release_date: Option<String>,
  pub genres: Vec<IGDBNamedResponse>,
  pub franchise: Option<IGDBNamedResponse>,
  pub franchises: Vec<IGDBNamedResponse>,
  pub alternative_names: Vec<IGDBNamedResponse>,
  pub involved_companies: Vec<IGDBInvolvedCompanyResponse>,
  pub collections: Vec<IGDBNamedResponse>,
  pub game_modes: Vec<IGDBNamedResponse>,
  pub platforms: Vec<Map<String, Value>>,
  pub age_ratings: Vec<Map<String, Value>>,
  pub expansions: Vec<IGDBRelatedGameResponse>,
  pub dlcs: Vec<IGDBRelatedGameResponse>,
  pub similar_games: Vec<IGDBRelatedGameResponse>,
}

pub type IGDBRomsResponse = Vec<IGDBRomResponse>;


// ? Search
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBSearchGameResponse {
  pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IGDBSearchResponse {
  pub name: String,
  pub game: IGDBSearchGameResponse,
}