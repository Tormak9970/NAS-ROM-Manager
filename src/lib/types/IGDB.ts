export type IGDBMetadataPlatform = {
  igdbId: string;
  name: string;
  abbreviation: string;
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
  coverUrl: string;
  thumbUrl: string;
}

export type IGDBMetadata = {
  totalRating: string;
  aggregatedRating: string;
  firstReleaseDate: number;
  genres: string[];
  franchises: string[];
  alternativeNames: string[];
  collections: string[];
  companies: string[];
  game_modes: string[];
  ageRatings: IGDBAgeRating[];
  platforms: IGDBMetadataPlatform[];
  expansions: IGDBRelatedGame[];
  dlcs: IGDBRelatedGame[];
  similarGames: IGDBRelatedGame[];
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