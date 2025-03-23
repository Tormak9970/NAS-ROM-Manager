export type IGDBMetadataPlatform = {
  igdbId: string;
  name: string;
  abbreviation: string;
}

export enum IGDB_WEBSITE_TYPES {
  SGDB = "SGDB",
  IGDB = "IGDB",
  OFFICIAL = "official",
  WIKIA = "wikia",
  WIKIPEDIA = "wikipedia",
  FACEBOOK = "facebook",
  TWITTER = "twitter",
  TWITCH = "twitch",
  INSTAGRAM = "instagram",
  YOUTUBE = "youtube",
  IPHONE = "iphone",
  IPAD = "ipad",
  ANDROID = "android",
  STEAM = "steam",
  REDDIT = "reddit",
  ITCH = "itch",
  EPICGAMES = "epicgames",
  GOG = "gog",
  DISCORD = "discord",
  BLUESKY = "bluesky",
};

export const RATINGS_COUNTRIES: Record<string, string> = {
    PEGI: "EU",
    ESRB: "U.S. & CA",
    CERO: "JP",
    USK: "DE",
    GRAC: "KR",
    CLASS_IND: "BR",
    ACB: "AU"
}

export type IGDBWebsite = {
  url: string;
  type: IGDB_WEBSITE_TYPES;
}

export type IGDBAgeRating = {
  rating: string;
  category: string;
  ratingCoverUrl: string;
}

export type IGDBRelatedGame = {
  id: string;
  name: string;
  slug: string;
  type: string;
}

export type IGDBMetadata = {
  totalRating: string;
  aggregatedRating: string;
  firstReleaseDate: number;
  keywords: string[];
  genres: string[];
  franchises: string[];
  alternativeNames: string[];
  collections: string[];
  developers: string[];
  publishers: string[];
  gameModes: string[];
  languages: string[];
  ageRatings: IGDBAgeRating[];
  platforms: IGDBMetadataPlatform[];
  dlcs: IGDBRelatedGame[];
  expansions: IGDBRelatedGame[];
  websites: IGDBWebsite[];
}

export type IGDBGame = {
  igdbId: string;
  slug: string;
  name: string;
  summary: string | null;
  coverUrl: string | null;
  thumbUrl: string | null;
  metadata: IGDBMetadata | null;
}

export type IGDBSearchResult = {
  igdbId: number;
  name: string;
}

export const NO_IGDB_RESULTS = "NO RESULTS";