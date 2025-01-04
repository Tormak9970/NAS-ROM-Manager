/**
 * Debounces a function by the provided interval.
 * @param func The function to debounce.
 * @param wait How long to wait before running the function after the last call.
 * @param immediate Whether to run the function immediately, then debounce, or debounce from the start.
 * @returns The debounced function.
 */
export function debounce(func: any, wait:number, immediate?: boolean) {
  let timeout:any|null;

  return function (...args: any[]) {
    const later = function () {
      timeout = null;
      if (!immediate) func(...args);
    };

    const callNow = immediate && !timeout;
    clearTimeout(timeout as any);
    timeout = setTimeout(later, wait);
    
    if (callNow) func(...args);
  }
}

/**
 * Throttles a function to only run every provided interval. From underscore souce code.
 * @param func The function to throttle.
 * @param wait How long to wait before running the function again.
 * @param immediate Whether to run the function immediately or not. 
 * @returns The throttled function.
 */
export function throttle(func: any, wait: number, immediate = false) {
  let context: any, args: any, result: any;
  let timeout: any = null;
  let previous: any = 0;

  const later = function() {
    previous = immediate === false ? 0 : new Date();
    timeout = null;
    result = func.apply(context, args);
    if (!timeout) context = args = null;
  };

  return function() {
    const now = new Date();
    if (!previous && immediate === false) previous = now;
    // @ts-ignore
    const remaining = wait - (now - previous);
    // @ts-ignore
    context = this;
    args = arguments;

    if (remaining <= 0 || remaining > wait) {
      if (timeout) {
        clearTimeout(timeout);
        timeout = null;
      }

      previous = now;
      result = func.apply(context, args);

      if (!timeout) context = args = null;
    } else if (!timeout && immediate !== false) {
      timeout = setTimeout(later, remaining);
    }

    return result;
  }
}

function prefixIfNeeded(time: number): string {
  return time < 10 ? "0" + time.toFixed(0) : time.toFixed(0)
}

/**
 * Formats a duration into an easy to read format.
 * @param totalSeconds The total time in seconds.
 * @returns The formatted time.
 */
export function formatTime(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / (60 * 60));
  const minutes = Math.floor((totalSeconds - hours * 60 * 60) / 60);
  const seconds = totalSeconds % 60;
  return `${hours !== 0 ? hours + ":" + prefixIfNeeded(minutes) : minutes}:${prefixIfNeeded(seconds)}`;
}

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

// cyrb53 (c) 2018 bryc (github.com/bryc). License: Public domain. Attribution appreciated.
// A fast and simple 64-bit (or 53-bit) string hash function with decent collision resistance.
// Largely inspired by MurmurHash2/3, but with a focus on speed/simplicity.
// See https://stackoverflow.com/questions/7616461/generate-a-hash-from-string-in-javascript/52171480#52171480
// https://github.com/bryc/code/blob/master/jshash/experimental/cyrb53.js
function cyrb64(str: string, seed = 0): [number, number] {
  let h1 = 0xdeadbeef ^ seed;
  let h2 = 0x41c6ce57 ^ seed;

  for(let i = 0, ch; i < str.length; i++) {
    ch = str.charCodeAt(i);
    h1 = Math.imul(h1 ^ ch, 2654435761);
    h2 = Math.imul(h2 ^ ch, 1597334677);
  }

  h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507);
  h1 ^= Math.imul(h2 ^ (h2 >>> 13), 3266489909);
  h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507);
  h2 ^= Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  // For a single 53-bit numeric return value we could return
  // 4294967296 * (2097151 & h2) + (h1 >>> 0);
  // but we instead return the full 64-bit value:
  return [h2 >>> 0, h1 >>> 0];
};

/**
 * Hashes a string. Output is always 14 characters.
 * @param str The string to hash.
 * @param seed An optional seed.
 * @returns The hash.
 */
export function hash64(str: string, seed = 0): string {
  const [h2, h1] = cyrb64(str, seed);
  return h2.toString(36).padStart(7, '0') + h1.toString(36).padStart(7, '0');
}

/**
 * Moves the element at the provided index to the target index.
 * @param array The array to swap the elements of.
 * @param moveIndex The index of the element to move.
 * @param toIndex The index to move to.
 * @returns A new array.
 */
export function swap<T>(array: T[], moveIndex: number, toIndex: number) {
  const item = array[moveIndex];
  const length = array.length;
  const diff = moveIndex - toIndex;

  if (diff > 0) {
    return [
      ...array.slice(0, toIndex),
      item,
      ...array.slice(toIndex, moveIndex),
      ...array.slice(moveIndex + 1, length)
    ];
  } else if (diff < 0) {
    const targetIndex = toIndex + 1;
    return [
      ...array.slice(0, moveIndex),
      ...array.slice(moveIndex + 1, targetIndex),
      item,
      ...array.slice(targetIndex, length)
    ];
  }
  return array;
}

/**
 * Clamps a value to the provided bounds.
 * @param value The value to clamp.
 * @param lower The lower bound.
 * @param upper The upper bound.
 */
export function clamp(value: number, lower: number, upper: number): number {
  return Math.min(upper, Math.max(value, lower));
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