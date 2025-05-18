async function checkLatestRelease(owner: string, repo: string) {
  const url = `https://api.github.com/repos/${owner}/${repo}/releases/latest`;

  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`GitHub API returned status ${response.status}`);
    }

    const data = await response.json();
    console.log("Latest release tag:", data.tag_name);

    // You can compare it to a stored tag version
    const currentTag = "v1.0.0";
    if (data.tag_name !== currentTag) {
      console.log("New release available:", data.tag_name);
    } else {
      console.log("No new release.");
    }
  } catch (error) {
    console.error("Error fetching release info:", error);
  }
}

// checkLatestRelease("owner", "repo"); // Replace with actual GitHub owner/repo