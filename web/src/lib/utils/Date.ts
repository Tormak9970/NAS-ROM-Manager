/**
 * Formats a dateString using the provided language.
 * @param dateString The date as a string.
 * @param lang The language.
 * @returns The formatted date.
 */
export function formatDate(dateString: string, lang: string): string {
  const formatter = new Intl.DateTimeFormat(lang, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });

  const date = new Date(dateString);
  return formatter.format(date);
}