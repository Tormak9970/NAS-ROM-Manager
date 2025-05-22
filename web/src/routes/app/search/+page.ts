export async function load({ url }) {
  return {
    textQuery: url.searchParams.get("textQuery"),
    system: url.searchParams.get("system"),
    genre: url.searchParams.get("genre"),
    publisher: url.searchParams.get("publisher"),
    developer: url.searchParams.get("developer"),
    format: url.searchParams.get("format"),
    startReleaseDate: url.searchParams.get("startReleaseDate"),
    endReleaseDate: url.searchParams.get("endReleaseDate"),
  }
};