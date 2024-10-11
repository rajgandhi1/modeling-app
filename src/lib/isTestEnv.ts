import { IS_PLAYWRIGHT_KEY } from '../../e2e/playwright/storageStates'

export const isTestEnv =
  globalThis.window?.localStorage.getItem(IS_PLAYWRIGHT_KEY) === 'true'
