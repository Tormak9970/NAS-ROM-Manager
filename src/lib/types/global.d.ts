/// <reference types="svelte" />
/// <reference types="vite/client" />

declare const APP_VERSION: string;

type DialogModalType = "INFO" | "WARNING" | "ERROR";

type ShowWarningOptions = {
  message: string;
  faster?: boolean;
}

type ShowInfoOptions = {
  message: string;
}

type ShowSnackbarOptions = {
  message: string;
  timeout?: number | null;
}

type SelectItem = {
  label: string;
  value: string;
}

declare module "svelte-icons/md/*.svelte";