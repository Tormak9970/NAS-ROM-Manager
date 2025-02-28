/**
 * Capitalizes each word in the phrase.
 * @param phrase The phrase to capitalize.
 */
export function capitalizeEachWord(phrase: string): string {
  const words = phrase.split(" ");
  return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join(" ");
}
/**
 * Normalizes a string.
 * @param value The string to normalize.
 */
export function normalizeString(value: string): string {
  const accentMap: Record<string, string> = {
    "ą": "a",
    "ć": "c",
    "ę": "e",
    "ł": "l",
    "ń": "n",
    "ó": "o",
    "ś": "s",
    "ź": "z",
    "ż": "z"
  };
  
  return value.normalize("NFD").replace(/[ąćęłńóśźż]/g, (matched) => accentMap[matched]);
}

/**
 * Capitalizes the first letter of each word.
 * @param word The _ delimited words.
 * @returns The space deleminited words.
 */
export function toUpperCaseSplit(word: string): string {
  if (word.includes("_")) {
    return word.split("_").map((w) => w.substring(0, 1).toUpperCase().concat(w.substring(1))).join(" ");
  } else {
    return word.substring(0, 1).toUpperCase().concat(word.substring(1));
  }
}

/**
 * Compares two strings. (from https://github.com/cemerick/jsdifflib)
 * @param first The first string.
 * @param second The second string.
 * @returns A float [0, 1] representing how similar the strings are.
 */
export function compareStrings(first: string, second: string) {
  first = first.replace(/\s+/g, '');
  second = second.replace(/\s+/g, '');

  if (first === second) return 1; // identical or empty
  if (first.length < 2 || second.length < 2) return 0; // if either is a 0-letter or 1-letter string

  let firstBigrams = new Map();
  for (let i = 0; i < first.length - 1; i++) {
    const bigram = first.substring(i, i + 2);
    const count = firstBigrams.has(bigram) ? firstBigrams.get(bigram) + 1 : 1;

    firstBigrams.set(bigram, count);
  }

  let intersectionSize = 0;
  for (let i = 0; i < second.length - 1; i++) {
    const bigram = second.substring(i, i + 2);
    const count = firstBigrams.has(bigram) ? firstBigrams.get(bigram) : 0;

    if (count > 0) {
      firstBigrams.set(bigram, count - 1);
      intersectionSize++;
    }
  }

  return (2.0 * intersectionSize) / (first.length + second.length - 2);
}