/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly GLM_DEBUG?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
