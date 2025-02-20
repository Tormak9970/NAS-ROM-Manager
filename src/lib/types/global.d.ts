/// <reference types="svelte" />
/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly NRM_BACKEND_VERSION: string;
  readonly NRM_BUILD_DATE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

declare const NRM_FRONTEND_VERSION: string;

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