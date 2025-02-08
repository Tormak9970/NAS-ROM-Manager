import type { ROM, RomUploadConfig } from "@types";
import { writable } from "svelte/store";

export const showControlledModal = writable(false);
export const controlledModalTitle = writable("");
export const controlledModalMessage = writable("");
export const controlledModalConfirmText = writable("");
export const controlledModalConfirm = writable(async () => {});
export const controlledModalCancelText = writable("");
export const controlledModalCancel = writable(async () => {});

export const showAddLibraryModal = writable(false);

export const showDownloadProgressModal = writable(false);
export const downloadProgressRom = writable<ROM | null>(null);

export const showAddRomModal = writable(false);
export const showUploadProgressModal = writable(false);
export const uploadProgressConfig = writable<RomUploadConfig | null>(null);

export const showEditRomModal = writable(false);
export const editIsPostUpload = writable(false);
export const romEditingId = writable<string | null>(null);