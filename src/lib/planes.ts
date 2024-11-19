import { DefaultPlaneStr } from 'clientSideScene/sceneEntities'
import { DefaultPlanes } from 'wasm-lib/kcl/bindings/DefaultPlanes'

/**
 * Converts a key of DefaultPlanes to the way it is written in KCL.
 */
export function defaultPlaneKeyToKcl(
  key: keyof DefaultPlanes
): DefaultPlaneStr {
  switch (key) {
    case 'negXy':
      return '-XY'
    case 'negXz':
      return '-XZ'
    case 'negYz':
      return '-YZ'
    case 'xy':
      return 'XY'
    case 'xz':
      return 'XZ'
    case 'yz':
      return 'YZ'
  }
}

/** A simple type guard to verify a string is a keyof DefaultPlanes */
export function isDefaultPlaneKey(
  key: string,
  planesObj: DefaultPlanes
): key is keyof DefaultPlanes {
  return key in planesObj
}
