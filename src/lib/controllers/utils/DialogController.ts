import { controlledModalCancel, controlledModalCancelText, controlledModalConfirm, controlledModalConfirmText, controlledModalMessage, controlledModalTitle, filePickerCancel, filePickerConfig, filePickerConfirm, showControlledModal, showFilePickerModal } from "@stores/Modals";
import type { FilePickerConfig } from "@types";

/**
 * Controller class for handling dialog modals.
 */
export class DialogController {
  /**
   * Displays a message with a single button.
   * @param title The title of the dialog modal.
   * @param message The message to display.
   * @param confirmText The text displayed in the button.
   */
  static async message(title: string, message: string, confirmText: string): Promise<boolean> {
    return new Promise((resolve) => {
      controlledModalTitle.set(title);
      controlledModalMessage.set(message);
      controlledModalConfirmText.set(confirmText);
      controlledModalConfirm.set(async () => resolve(true));
      controlledModalCancelText.set("");
      controlledModalCancel.set(async () => {});

      showControlledModal.set(true);
    });
  }

  /**
   * Asks the user for input on a decision.
   * @param title The title of the dialog modal.
   * @param message The message of the dialog modal.
   * @param confirmText The text displayed for the confirm action.
   * @param cancelText The text displayed for the cancel action.
   */
  static async ask(title: string, message: string, confirmText: string, cancelText: string): Promise<boolean> {
    return new Promise((resolve) => {
      controlledModalTitle.set(title);
      
      controlledModalMessage.set(message);
      controlledModalConfirmText.set(confirmText);
      controlledModalConfirm.set(async () => resolve(true));
      controlledModalCancelText.set(cancelText);
      controlledModalCancel.set(async () => resolve(false));

      showControlledModal.set(true);
    });
  }

  /**
   * Prompts the user to select file/folder paths.
   * @param config The filepicker config.
   * @returns The selected paths.
   */
  static async openFilePicker(config: FilePickerConfig): Promise<string[]> {
    return new Promise((resolve) => {
      filePickerConfig.set(config);
      filePickerConfirm.set(async (paths: string[]) => resolve(paths));
      filePickerCancel.set(async () => resolve([]));

      showFilePickerModal.set(true);
    });
  }
}