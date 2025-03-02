use phf::phf_map;
use serde::{Deserialize, Serialize};

pub const PLATFORMS_FIELDS: [&'static str; 8] = [
  "id",
  "name",
  "category",
  "generation",
  "url",
  "platform_family.name",
  "platform_family.slug",
  "platform_logo.url",
];

pub const GAMES_FIELDS: [&'static str; 49] = [
  "id",
  "name",
  "slug",
  "summary",
  "total_rating",
  "aggregated_rating",
  "first_release_date",
  "artworks.url",
  "cover.url",
  "screenshots.url",
  "platforms.id",
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
  "expanded_games.id",
  "expanded_games.slug",
  "expanded_games.name",
  "expanded_games.cover.url",
  "dlcs.id",
  "dlcs.name",
  "dlcs.slug",
  "dlcs.cover.url",
  "remakes.id",
  "remakes.slug",
  "remakes.name",
  "remakes.cover.url",
  "remasters.id",
  "remasters.slug",
  "remasters.name",
  "remasters.cover.url",
  "ports.id",
  "ports.slug",
  "ports.name",
  "ports.cover.url",
  "similar_games.id",
  "similar_games.slug",
  "similar_games.name",
  "similar_games.cover.url",
  "age_ratings.rating",
  "videos.video_id",
];

pub const SEARCH_FIELDS: [&'static str; 2] = ["game.id", "name"];


pub static IGDB_PLATFORM_CATEGORIES: phf::Map<&'static str, &'static str> = phf_map! {
  "0" => "Unknown",
  "1" => "Console",
  "2" => "Arcade",
  "3" => "Platform",
  "4" => "Operative System",
  "5" => "Portable Console",
  "6" => "Computer",
};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IGDBAgeRating<'a> {
  pub rating: &'a str,
  pub category: &'a str,
  pub ratingCoverUrl: &'a str,
}

pub static IGDB_AGE_RATINGS: phf::Map<&'static str, IGDBAgeRating<'static>> = phf_map! {
  "1" => IGDBAgeRating {
    rating: "Three",
    category: "PEGI",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/pegi/pegi_3.png",
  },
  "2" => IGDBAgeRating {
    rating: "Seven",
    category: "PEGI",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/pegi/pegi_7.png",
  },
  "3" => IGDBAgeRating {
    rating: "Twelve",
    category: "PEGI",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/pegi/pegi_12.png",
  },
  "4" => IGDBAgeRating {
    rating: "Sixteen",
    category: "PEGI",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/pegi/pegi_16.png",
  },
  "5" => IGDBAgeRating {
    rating: "Eighteen",
    category: "PEGI",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/pegi/pegi_18.png",
  },
  "6" => IGDBAgeRating {
    rating: "RP",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_rp.png",
  },
  "7" => IGDBAgeRating {
    rating: "EC",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_ec.png",
  },
  "8" => IGDBAgeRating {
    rating: "E",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_e.png",
  },
  "9" => IGDBAgeRating {
    rating: "E10",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_e10.png",
  },
  "10" => IGDBAgeRating {
    rating: "T",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_t.png",
  },
  "11" => IGDBAgeRating {
    rating: "M",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_m.png",
  },
  "12" => IGDBAgeRating {
    rating: "AO",
    category: "ESRB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/esrb/esrb_ao.png",
  },
  "13" => IGDBAgeRating {
    rating: "CERO_A",
    category: "CERO",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/cero/cero_a.png",
  },
  "14" => IGDBAgeRating {
    rating: "CERO_B",
    category: "CERO",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/cero/cero_b.png",
  },
  "15" => IGDBAgeRating {
    rating: "CERO_C",
    category: "CERO",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/cero/cero_c.png",
  },
  "16" => IGDBAgeRating {
    rating: "CERO_D",
    category: "CERO",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/cero/cero_d.png",
  },
  "17" => IGDBAgeRating {
    rating: "CERO_Z",
    category: "CERO",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/cero/cero_z.png",
  },
  "18" => IGDBAgeRating {
    rating: "USK_0",
    category: "USK",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/usk/usk_0.png",
  },
  "19" => IGDBAgeRating {
    rating: "USK_6",
    category: "USK",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/usk/usk_6.png",
  },
  "20" => IGDBAgeRating {
    rating: "USK_12",
    category: "USK",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/usk/usk_12.png",
  },
  "21" => IGDBAgeRating {
    rating: "USK_16",
    category: "USK",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/usk/usk_16.png",
  },
  "22" => IGDBAgeRating {
    rating: "USK_18",
    category: "USK",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/usk/usk_18.png",
  },
  "23" => IGDBAgeRating {
    rating: "GRAC_ALL",
    category: "GRAC",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/grac/grac_all.png",
  },
  "24" => IGDBAgeRating {
    rating: "GRAC_Twelve",
    category: "GRAC",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/grac/grac_12.png",
  },
  "25" => IGDBAgeRating {
    rating: "GRAC_Fifteen",
    category: "GRAC",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/grac/grac_15.png",
  },
  "26" => IGDBAgeRating {
    rating: "GRAC_Eighteen",
    category: "GRAC",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/grac/grac_18.png",
  },
  "27" => IGDBAgeRating {
    rating: "GRAC_TESTING",
    category: "GRAC",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/grac/grac_testing.png",
  },
  "28" => IGDBAgeRating {
    rating: "CLASS_IND_L",
    category: "CLASS_IND",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_l.png",
  },
  "29" => IGDBAgeRating {
    rating: "CLASS_IND_Ten",
    category: "CLASS_IND",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_10.png",
  },
  "30" => IGDBAgeRating {
    rating: "CLASS_IND_Twelve",
    category: "CLASS_IND",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/class_ind/class_ind_12.png",
  },
  "31" => IGDBAgeRating {
    rating: "ACB_G",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_g.png",
  },
  "32" => IGDBAgeRating {
    rating: "ACB_PG",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_pg.png",
  },
  "33" => IGDBAgeRating {
    rating: "ACB_M",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_m.png",
  },
  "34" => IGDBAgeRating {
    rating: "ACB_MA15",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_ma15.png",
  },
  "35" => IGDBAgeRating {
    rating: "ACB_R18",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_r18.png",
  },
  "36" => IGDBAgeRating {
    rating: "ACB_RC",
    category: "ACB",
    ratingCoverUrl: "https://www.igdb.com/icons/rating_icons/acb/acb_rc.png",
  },
};