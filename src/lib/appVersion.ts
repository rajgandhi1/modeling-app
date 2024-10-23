import { NODE_ENV } from 'env'
import { isDesktop } from './isDesktop'
import { isTestEnv } from './isTestEnv'

/** Version number of the app */
export const APP_VERSION =
  isTestEnv && NODE_ENV === 'development'
    ? '11.22.33'
    : isDesktop()
    ? // @ts-ignore
      window.electron.packageJson.version
    : 'main'
