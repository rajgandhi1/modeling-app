import { useEffect } from 'react'
import { ActionButton } from './ActionButton'
import { StatusBarItemType } from './statusBar/statusBarTypes'
import Tooltip, { TooltipProps } from './Tooltip'
import { ActionIcon } from './ActionIcon'

export function StatusBar({
  globalItems,
  localItems,
}: {
  globalItems: StatusBarItemType[]
  localItems: StatusBarItemType[]
}) {
  useEffect(() => {
    console.log('items', {
      globalItems,
      localItems,
    })
  }, [])
  return (
    <footer
      id="statusbar"
      className="relative z-10 flex justify-between items-center bg-chalkboard-20 dark:bg-chalkboard-90 text-chalkboard-80 dark:text-chalkboard-30 border-t border-t-chalkboard-30 dark:border-t-chalkboard-80"
    >
      <menu id="statusbar-globals" className="flex items-stretch">
        {globalItems.map((item, index) => (
          <StatusBarItem key={item.id} {...item} position={'left'} />
        ))}
      </menu>
      <menu id="statusbar-locals" className="flex items-stretch">
        {localItems.map((item, index) => (
          <StatusBarItem
            key={item.id}
            {...item}
            position={index === localItems.length - 1 ? 'right' : 'middle'}
          />
        ))}
      </menu>
    </footer>
  )
}

function StatusBarItem(
  props: StatusBarItemType & { position: 'left' | 'middle' | 'right' }
) {
  const defaultClassNames = `px-2 py-1 text-xs text-chalkboard-80 dark:text-chalkboard-30 rounded-none border-none hover:bg-chalkboard-30 dark:hover:bg-chalkboard-80 focus:bg-chalkboard-30 dark:focus:bg-chalkboard-80 hover:text-chalkboard-100 dark:hover:text-chalkboard-10 focustext-chalkboard-100 dark:focus:text-chalkboard-10  focus:outline-none focus-visible:ring-2 focus:ring-primary focus:ring-opacity-50`
  const tooltipPosition: TooltipProps['position'] =
    props.position === 'middle' ? 'top' : `top-${props.position}`

  switch (props.element) {
    case 'button':
      return (
        <ActionButton
          Element="button"
          iconStart={
            props.icon && {
              icon: props.icon,
              iconClassName: props.icon === 'loading' ? 'animate-spin' : '',
              bgClassName: 'bg-transparent dark:bg-transparent',
            }
          }
          className={defaultClassNames + ' ' + props.className}
          data-testid={props['data-testid']}
        >
          {props.label && (
            <span className={props.hideLabel ? 'sr-only' : ''}>
              {props.label}
            </span>
          )}
          {props.toolTip && (
            <Tooltip {...props.toolTip} position={tooltipPosition} />
          )}
        </ActionButton>
      )
    case 'text':
      return (
        <div
          role="tooltip"
          className={defaultClassNames + ' ' + props.className}
        >
          {props.icon && (
            <ActionIcon
              icon={props.icon}
              iconClassName={props.icon === 'loading' ? 'animate-spin' : ''}
              bgClassName="bg-transparent dark:bg-transparent"
            />
          )}
          {props.label && (
            <span className={props.hideLabel ? 'sr-only' : ''}>
              {props.label}
            </span>
          )}
          {props.toolTip && (
            <Tooltip {...props.toolTip} position={tooltipPosition} />
          )}
        </div>
      )
    default:
      return (
        <ActionButton
          Element={props.element}
          to={props.href}
          iconStart={
            props.icon && {
              icon: props.icon,
              bgClassName: 'bg-transparent dark:bg-transparent',
            }
          }
          className={defaultClassNames + ' ' + props.className}
          data-testid={props['data-testid']}
        >
          {props.label && (
            <span className={props.hideLabel ? 'sr-only' : ''}>
              {props.label}
            </span>
          )}
          {props.toolTip && (
            <Tooltip {...props.toolTip} position={tooltipPosition} />
          )}
        </ActionButton>
      )
  }
}
