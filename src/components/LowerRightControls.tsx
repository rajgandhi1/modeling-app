import { CustomIcon } from 'components/CustomIcon'
import Tooltip from 'components/Tooltip'
import { PATHS } from 'lib/paths'
import { NetworkHealthIndicator } from 'components/NetworkHealthIndicator'
import { HelpMenu } from './HelpMenu'
import { Link, useLocation } from 'react-router-dom'
import { useAbsoluteFilePath } from 'hooks/useAbsoluteFilePath'
import { CoreDumpManager } from 'lib/coredump'
import { NetworkMachineIndicator } from './NetworkMachineIndicator'

export function LowerRightControls({
  children,
  coreDumpManager,
}: {
  children?: React.ReactNode
  coreDumpManager?: CoreDumpManager
}) {
  const location = useLocation()
  const filePath = useAbsoluteFilePath()
  const linkOverrideClassName =
    '!text-chalkboard-70 hover:!text-chalkboard-80 dark:!text-chalkboard-40 dark:hover:!text-chalkboard-30'

  return (
    <section className="absolute bottom-2 right-2 flex flex-col items-end gap-3 pointer-events-none">
      {children}
      <menu className="flex items-center justify-end gap-3 pointer-events-auto">
        <NetworkMachineIndicator className={linkOverrideClassName} />
      </menu>
    </section>
  )
}
