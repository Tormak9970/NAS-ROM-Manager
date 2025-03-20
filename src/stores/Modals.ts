import type { FilePickerConfig, ROM, RomUploadConfig } from "@types";
import { localStorageWritable } from "@utils";
import { writable } from "svelte/store";

export const showControlledModal = writable(false);
export const controlledModalTitle = writable("");
export const controlledModalMessage = writable("");
export const controlledModalConfirmText = writable("");
export const controlledModalConfirm = writable(async () => {});
export const controlledModalCancelText = writable("");
export const controlledModalCancel = writable(async () => {});

export const showFilePickerModal = writable(false);
export const filePickerConfig = writable<FilePickerConfig | null>(null);
export const filePickerConfirm = writable(async (paths: string[]) => {});
export const filePickerCancel = writable(async () => {});

export const showEditLibraryModal = writable(false);
export const isFirstSetup = writable(true);

export const showEditRouteOrderModal = writable(false);
export const isLandscapeRoutes = writable(true);

export const showDownloadProgressModal = writable(false);
export const downloadProgressRom = writable<ROM | null>(null);

export const showAddRomModal = writable(false);
export const addRomSystem = localStorageWritable<string>("add-rom-selected-system", "");

export const showUploadProgressModal = writable(false);
export const uploadProgressConfig = writable<RomUploadConfig | null>(null);

export const showEditRomModal = writable(false);
export const editIsPostUpload = writable(false);
export const romEditingId = writable<string | null>(null);

export const showLoadingModal = writable(false);
export const loadingModalMessage = writable("");