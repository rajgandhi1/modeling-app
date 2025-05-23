import type { UnitLength } from '@rust/kcl-lib/bindings/ModelingCmd'

import {
  changeDefaultUnits,
  unitAngleToUnitAng,
  unitLengthToUnitLen,
} from '@src/lang/wasm'
import { DEFAULT_DEFAULT_LENGTH_UNIT } from '@src/lib/constants'

/**
 * Create a new KCL file with the given initial content and default length unit.
 * @returns KCL string
 */
export function newKclFile(
  initialContent: string | undefined,
  defaultLengthUnit: UnitLength
): string | Error {
  // If we're given initial content, we're loading a file that should already
  // have units in it.  Don't modify it.
  if (initialContent !== undefined) {
    return initialContent
  }
  // If the default length unit is the same as the default default length unit,
  // there's no need to add the attribute.
  if (defaultLengthUnit === DEFAULT_DEFAULT_LENGTH_UNIT) {
    return ''
  }

  return changeDefaultUnits(
    '',
    unitLengthToUnitLen(defaultLengthUnit),
    unitAngleToUnitAng(undefined)
  )
}
