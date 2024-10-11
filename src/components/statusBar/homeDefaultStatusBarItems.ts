import openWindow from 'lib/openWindow'
import { StatusBarItemType } from './statusBarTypes'
import { reportRejection } from 'lib/trap'
import { CoreDumpManager } from 'lib/coredump'
import toast from 'react-hot-toast'
import { coreDump } from 'lang/wasm'
import { APP_VERSION } from 'lib/constants'
import { Location } from 'react-router-dom'
import { PATHS } from 'lib/paths'

export const homeDefaultStatusBarItems = ({
  coreDumpManager,
  location,
}: {
  coreDumpManager?: CoreDumpManager
  location: Location
}): StatusBarItemType[] => [
  {
    id: 'version',
    element: 'externalLink',
    label: `v${APP_VERSION}`,
    href: `https://github.com/KittyCAD/modeling-app/releases/tag/v${APP_VERSION}`,
    toolTip: {
      children: 'View the release notes on GitHub',
    },
  },
  {
    id: 'report-bug',
    element: 'button',
    label: 'Report a bug',
    onClick: (event) => reportBug(event, { coreDumpManager }),
    toolTip: {
      children: 'Send your current app state to the developers for debugging',
    },
  },
  {
    id: 'settings',
    element: 'link',
    icon: 'settings',
    href:
      '.' +
      PATHS.SETTINGS +
      (location.pathname.includes(PATHS.FILE) ? '?tab=project' : ''),
    'data-testid': 'settings-link',
    label: 'Settings',
    hideLabel: true,
    toolTip: {
      children: 'Settings',
    },
  },
]

function reportBug(
  event: {
    preventDefault: () => void
    stopPropagation: () => void
  },
  dependencies: {
    coreDumpManager: CoreDumpManager | undefined
  }
) {
  event?.preventDefault()
  event?.stopPropagation()
  const { coreDumpManager } = dependencies

  if (!coreDumpManager) {
    // open default reporting option
    openWindow(
      'https://github.com/KittyCAD/modeling-app/issues/new/choose'
    ).catch(reportRejection)
  } else {
    toast
      .promise(
        coreDump(coreDumpManager, true),
        {
          loading: 'Preparing bug report...',
          success: 'Bug report opened in new window',
          error: 'Unable to export a core dump. Using default reporting.',
        },
        {
          success: {
            // Note: this extended duration is especially important for Playwright e2e testing
            // default duration is 2000 - https://react-hot-toast.com/docs/toast#default-durations
            duration: 6000,
          },
        }
      )
      .catch((err: Error) => {
        if (err) {
          openWindow(
            'https://github.com/KittyCAD/modeling-app/issues/new/choose'
          ).catch(reportRejection)
        }
      })
  }
}
