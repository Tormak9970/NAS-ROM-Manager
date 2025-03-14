export type IGDBMetadataPlatform = {
  igdbId: string;
  name: string;
  abbreviation: string;
}

export type IGDBWebsite = {
  url: string;
  type: string;
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
  igdbId: string;
  name: string;
}

export const NO_IGDB_RESULTS = "NO RESULTS";