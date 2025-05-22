import { controlledModalCancel, controlledModalCancelText, controlledModalConfirm, controlledModalConfirmText, controlledModalIsWarning, controlledModalMessage, controlledModalTitle, filePickerCancel, filePickerConfig, filePickerConfirm, showControlledModal, showFilePickerModal } from "@stores/Modals";
import type { FilePickerConfig } from "@types";

/**
 * Dialog Modal Service.
 */
export class DialogService {
  /**
   * Displays a message with a single button.
   * @param title The title of the dialog modal.
   * @param message The message to display.
   * @param confirmText The text displayed in the button.
   * @param isWarning True if this is a warning prompt.
   */
  static async message(title: string, message: string, confirmText: string, isWarning = false): Promise<boolean> {
    return new Promise((resolve) => {
      controlledModalTitle.set(title);
      controlledModalMessage.set(message);
      controlledModalConfirmText.set(confirmText);
      controlledModalConfirm.set(async () => resolve(true));
      controlledModalCancelText.set("");
      controlledModalCancel.set(async () => {});
      controlledModalIsWarning.set(isWarning);

      showControlledModal.set(true);
    });
  }

  /**
   * Asks the user for input on a decision.
   * @param title The title of the dialog modal.
   * @param message The message of the dialog modal.
   * @param confirmText The text displayed for the confirm action.
   * @param cancelText The text displayed for the cancel action.
   * @param isWarning True if this is a warning prompt.
   */
  static async ask(title: string, message: string, confirmText: string, cancelText: string, isWarning = false): Promise<boolean> {
    return new Promise((resolve) => {
      controlledModalTitle.set(title);
      
      controlledModalMessage.set(message);
      controlledModalConfirmText.set(confirmText);
      controlledModalConfirm.set(async () => resolve(true));
      controlledModalCancelText.set(cancelText);
      controlledModalCancel.set(async () => resolve(false));
      controlledModalIsWarning.set(isWarning);

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