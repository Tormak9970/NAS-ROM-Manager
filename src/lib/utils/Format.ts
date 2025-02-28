/**
 * Formats a file's size using Kb, Mb, Gb, etc.
 * @param fileSize The file size.
 * @returns The formatted size.
 */
export function formatFileSize(fileSize: number): string {
  if (fileSize < 1000000) {
    return (fileSize / 1000).toFixed(1) + " KB";
  } else if (fileSize < 1000000000) {
    return (fileSize / 1000000).toFixed(1) + " MB";
  } else if (fileSize < 1000000000000) {
    return (fileSize / 1000000000).toFixed(1) + " GB";
  } else {
    return (fileSize / 1000000000000).toFixed(1) + " TB";
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