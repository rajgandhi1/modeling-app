import type { Extension, Range } from '@codemirror/state'
import type { DecorationSet, ViewUpdate } from '@codemirror/view'
import {
  Decoration,
  EditorView,
  ViewPlugin,
  WidgetType,
} from '@codemirror/view'
import type { LanguageServerPlugin } from './lsp'
import { lspColorUpdateEvent } from './annotation'
import { isArray } from '../lib/utils'
import { offsetToPos, posToOffset, posToOffsetOrZero } from './util'
import type * as LSP from 'vscode-languageserver-protocol'

interface PickerState {
  from: number
  to: number
  red: number
  green: number
  blue: number
  alpha: number
}

export interface WidgetOptions extends PickerState {
  color: string
}

export type ColorData = Omit<WidgetOptions, 'from' | 'to'>

const pickerState = new WeakMap<HTMLInputElement, PickerState>()

function rgbaToHex(color: LSP.Color): string {
  return `#${decimalToHex(color.red)}${decimalToHex(color.green)}${decimalToHex(
    color.blue
  )}`
}

function decimalToHex(decimal: number): string {
  const hex = decimal.toString(16)
  return hex.length === 1 ? '0' + hex : hex
}

function hexToRGBComponents(hex: string): number[] {
  const r = hex.slice(1, 3)
  const g = hex.slice(3, 5)
  const b = hex.slice(5, 7)
  return [parseInt(r, 16) /255, parseInt(g, 16)/255, parseInt(b, 16) /255]
}

async function discoverColorsViaLsp(
  view: EditorView,
  plugin: LanguageServerPlugin
): Promise<WidgetOptions | Array<WidgetOptions> | null> {
  const responses = await plugin.requestDocumentColors()

  if (!responses) {
    return null
  }

  let colors: Array<WidgetOptions> = []
  for (const color of responses) {
    if (!color.range) {
      continue
    }

    const { start, end } = color.range

    let from = posToOffset(view.state.doc, start)
    let to = posToOffset(view.state.doc, end)
    if (from === null) return null
    if (to === null) return null

    if (!from || !to) {
      continue
    }

    if (!color.color) {
      continue
    }

    colors.push({
      color: rgbaToHex(color.color),
      ...color.color,
      from,
      to,
    })
  }
  return colors
}

async function colorPickersDecorations(
  view: EditorView,
  plugin: LanguageServerPlugin
): Promise<DecorationSet> {
  const widgets: Array<Range<Decoration>> = []

  const maybeWidgetOptions = await discoverColorsViaLsp(view, plugin)

  if (!maybeWidgetOptions) {
    return Decoration.none
  }

  if (!isArray(maybeWidgetOptions)) {
    widgets.push(
      Decoration.widget({
        widget: new ColorPickerWidget(maybeWidgetOptions),
        side: 1,
      }).range(maybeWidgetOptions.from)
    )

    return Decoration.set(widgets)
  }

  for (const wo of maybeWidgetOptions) {
    widgets.push(
      Decoration.widget({
        widget: new ColorPickerWidget(wo),
        side: 1,
      }).range(wo.from)
    )
  }

  return Decoration.set(widgets)
}

export const wrapperClassName = 'cm-css-color-picker-wrapper'

class ColorPickerWidget extends WidgetType {
  private readonly state: PickerState
  private readonly color: string

  constructor({ color, ...state }: WidgetOptions) {
    super()
    this.state = state
    this.color = color
  }

  eq(other: ColorPickerWidget) {
    return (
      other.color === this.color &&
      other.state.from === this.state.from &&
      other.state.to === this.state.to &&
      other.state.alpha === this.state.alpha
    )
  }

  toDOM() {
    const picker = document.createElement('input')
    pickerState.set(picker, this.state)
    picker.type = 'color'
    picker.value = this.color

    const wrapper = document.createElement('span')
    wrapper.appendChild(picker)
    wrapper.className = wrapperClassName

    return wrapper
  }

  ignoreEvent() {
    return false
  }
}

export const colorPickerTheme = EditorView.baseTheme({
  [`.${wrapperClassName}`]: {
    display: 'inline-block',
    outline: '1px solid #eee',
    marginRight: '0.6ch',
    height: '1em',
    width: '1em',
    transform: 'translateY(1px)',
  },
  [`.${wrapperClassName} input[type="color"]`]: {
    cursor: 'pointer',
    height: '100%',
    width: '100%',
    padding: 0,
    border: 'none',
    '&::-webkit-color-swatch-wrapper': {
      padding: 0,
    },
    '&::-webkit-color-swatch': {
      border: 'none',
    },
    '&::-moz-color-swatch': {
      border: 'none',
    },
  },
})

export const makeColorPicker = (plugin: ViewPlugin<LanguageServerPlugin>) =>
  ViewPlugin.fromClass(
    class ColorPickerViewPlugin {
      decorations: DecorationSet = Decoration.none
      plugin: LanguageServerPlugin | null

      constructor(view: EditorView) {
        const value = view.plugin(plugin)
        this.plugin = value
        if (!this.plugin) return
        colorPickersDecorations(view, this.plugin).then((decorations) => {
          this.decorations = decorations
        })
      }

      async update(update: ViewUpdate) {
        if (!this.plugin) return

        if (!(update.docChanged || update.viewportChanged)) return

        this.decorations = await colorPickersDecorations(
          update.view,
          this.plugin
        )
      }
    },
    {
      decorations: (v) => v.decorations,
      eventHandlers: {
        change: async (e, view) => {
          const value = view.plugin(plugin)
          if (!value) return

          const target = e.target as HTMLInputElement
          if (
            target.nodeName !== 'INPUT' ||
            !target.parentElement ||
            !target.parentElement.classList.contains(wrapperClassName)
          ) {
            return false
          }

          const data = pickerState.get(target)!
          const converted = target.value + data.alpha

          const color = hexToRGBComponents(converted)

          const responses = await value.requestColorPresentation(
            {
              red: color[0],
              green: color[1],
              blue: color[2],
              alpha: data.alpha,
            },
            {
              start: offsetToPos(view.state.doc, data.from),
              end: offsetToPos(view.state.doc, data.to),
            }
          )

          if (!responses) {
            return false
          }

          if (responses.length === 0) {
            return false
          }

          for (const resp of responses) {
            let changes = {
              from: data.from,
              to: data.to,
              insert: resp.label,
            }

            if (resp.textEdit) {
              changes = {
                from: posToOffsetOrZero(
                  view.state.doc,
                  resp.textEdit.range.start
                ),
                to: posToOffsetOrZero(view.state.doc, resp.textEdit.range.end),
                insert: resp.textEdit.newText,
              }
            }

            view.dispatch({
              changes: changes,
              annotations: [lspColorUpdateEvent],
            })
          }

          return true
        },
      },
    }
  )

export default function lspColorsExt(
  plugin: ViewPlugin<LanguageServerPlugin>
): Extension {
  return [makeColorPicker(plugin), colorPickerTheme]
}
