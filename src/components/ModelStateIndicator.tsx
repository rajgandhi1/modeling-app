import { useEngineCommands } from './EngineCommands'
import { Spinner } from './Spinner'
import { CustomIcon } from './CustomIcon'
import { StatusBarItemType } from './statusBar/statusBarTypes'

export const useModelStateStatus = (): StatusBarItemType => {
  const [commands] = useEngineCommands()
  const lastCommandType = commands[commands.length - 1]?.type

  let icon: StatusBarItemType['icon'] = 'loading'
  const baseDataTestId = 'model-state-indicator'
  let dataTestId = baseDataTestId

  if (lastCommandType === 'receive-reliable') {
    icon = 'checkmark'
    dataTestId = `${baseDataTestId}-receive-reliable`
  } else if (lastCommandType === 'execution-done') {
    icon = 'checkmark'
    dataTestId = `${baseDataTestId}-execution-done`
  } else if (lastCommandType === 'export-done') {
    icon = 'checkmark'
    dataTestId = `${baseDataTestId}-export-done`
  }

  return {
    id: 'model-state-indicator',
    label: '',
    icon,
    toolTip: {
      children: 'Model state indicator',
    },
    element: 'button',
    onClick: () => {},
    'data-testid': dataTestId,
  }
}

export const ModelStateIndicator = () => {
  const [commands] = useEngineCommands()

  const lastCommandType = commands[commands.length - 1]?.type

  let className = 'w-6 h-6 '
  let icon = <Spinner className={className} />
  let dataTestId = 'model-state-indicator'

  if (lastCommandType === 'receive-reliable') {
    className +=
      'bg-chalkboard-20 dark:bg-chalkboard-80 !group-disabled:bg-chalkboard-30 !dark:group-disabled:bg-chalkboard-80 rounded-sm bg-succeed-10/30 dark:bg-succeed'
    icon = (
      <CustomIcon
        data-testid={dataTestId + '-receive-reliable'}
        name="checkmark"
      />
    )
  } else if (lastCommandType === 'execution-done') {
    className +=
      'border-6 border border-solid border-chalkboard-60 dark:border-chalkboard-80 bg-chalkboard-20 dark:bg-chalkboard-80 !group-disabled:bg-chalkboard-30 !dark:group-disabled:bg-chalkboard-80 rounded-sm bg-succeed-10/30 dark:bg-succeed'
    icon = (
      <CustomIcon
        data-testid={dataTestId + '-execution-done'}
        name="checkmark"
      />
    )
  } else if (lastCommandType === 'export-done') {
    className +=
      'border-6 border border-solid border-chalkboard-60 dark:border-chalkboard-80 bg-chalkboard-20 dark:bg-chalkboard-80 !group-disabled:bg-chalkboard-30 !dark:group-disabled:bg-chalkboard-80 rounded-sm bg-succeed-10/30 dark:bg-succeed'
    icon = (
      <CustomIcon data-testid={dataTestId + '-export-done'} name="checkmark" />
    )
  }

  return (
    <div className={className} data-testid="model-state-indicator">
      {icon}
    </div>
  )
}
