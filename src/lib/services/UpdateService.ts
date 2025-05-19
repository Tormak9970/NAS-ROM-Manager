import { showUpdateModal } from "@stores/Modals";
import { updateManifest } from "@stores/Update";
import { LogService } from "./utils/LogService";

async function checkLatestRelease(owner: string, repo: string): Promise<Update> {
  const url = `https://api.github.com/repos/${owner}/${repo}/releases/latest`;

  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`GitHub API returned status ${response.status}`);
    }

    const data = await response.json();

    if (data.tag_name !== NRM_FRONTEND_VERSION) {
      LogService.log("New release available:", data.tag_name);

      return {
        available: true,
        name: data.name,
        releaseDate: data.published_at,
        currentVersion: NRM_FRONTEND_VERSION,
        version: data.tag_name,
        body: data.body,
      }
    } else {
      LogService.log("No new release.");

      return {
        available: false,
        name: "",
        releaseDate: "",
        currentVersion: "",
        version: "",
        body: "",
      }
    }
  } catch (error) {
    LogService.error("Error fetching release info:", error);
    
    return {
      available: false,
      name: "",
      releaseDate: "",
      currentVersion: "",
      version: "",
      body: "",
    }
  }
}

/**
 * The Update Notification Service.
 */
export class UpdateService {
  /**
   * Checks if there is an update available.
   */
  static async checkForUpdate() {
    const releaseInfo = await checkLatestRelease("Tormak9970", "Steam-Art-Manager");

    if (releaseInfo.available) {
      updateManifest.set(releaseInfo);
      showUpdateModal.set(true);
    }
  }
}