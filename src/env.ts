// It turns out import.meta.env is a really fucky env var passing method.
// It's purely generated by Vite and nothing else.
// For Jest tests, we use babel to deal with it (it's a Syntax error otherwise)
// @ts-ignore: TS1343
const env = window.electron?.process.env ?? import.meta.env

export const NODE_ENV = env.NODE_ENV as string | undefined
export const VITE_KC_API_WS_MODELING_URL = env.VITE_KC_API_WS_MODELING_URL as
  | string
  | undefined
export const VITE_KITTYCAD_API_BASE_URL = env.VITE_KITTYCAD_API_BASE_URL
export const VITE_KC_SITE_BASE_URL = env.VITE_KC_SITE_BASE_URL
export const VITE_KC_SITE_APP_URL = env.VITE_KC_SITE_APP_URL
export const VITE_KC_CONNECTION_TIMEOUT_MS =
  env.VITE_KC_CONNECTION_TIMEOUT_MS as string | undefined
export const VITE_KITTYCAD_API_TOKEN = env.VITE_KITTYCAD_API_TOKEN as
  | string
  | undefined
export const PROD = env.PROD as string | undefined
export const TEST = env.TEST as string | undefined
export const DEV = env.DEV as string | undefined
export const CI = env.CI as string | undefined
