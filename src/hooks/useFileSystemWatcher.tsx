import { useCallback, useEffect, useState } from 'react'
import fs from 'node:fs'

type Path = string

type WatcherCallback = (eventType: string, path: string) => void

// Not having access to NodeJS functions has influenced the design a little.
// There is some indirection going on because we can only pass data between
// the NodeJS<->Browser boundary. The actual functions need to run on the
// NodeJS side. Because EventEmitters come bundled with their listener
// methods it complicates things because we can't just do 
// watcher.addListener(() => { ... }).

export const useFileSystemWatcher = (callback: (path: Path) => void, dependencyArray: Path[]): void => {
  // Used to track if dependencyArrray changes.
  const [dependencyArrayTracked, setDependencyArrayTracked] = useState<Path[]>([])

  // On component teardown obliterate all watchers.
  useEffect(() => {
    return () => {
      window.electron.watchFileObliterate()
    }
  }, [])

  function difference<T>(l1: T[], l2: T[]): [T[], T[]] {
    return [
      l1.filter((x) => Boolean(!l2.find((x2) => x2 === x))),
      l1.filter((x) => Boolean(l2.find((x2) => x2 === x)))
    ]
  }

  // Removing 1 watcher at a time is only possible because in a filesystem,
  // a path is unique (there can never be two paths with the same name).
  // Otherwise we would have to obliterate() the whole list and reconstruct it.
  useEffect(() => {
    const [pathsRemoved, pathsRemaining] = difference(dependencyArrayTracked, dependencyArray)
    for (let path of pathsRemoved) {
      window.electron.watchFileOff(path)
    }
    const [pathsAdded] = difference(dependencyArray, dependencyArrayTracked)
    for (let path of pathsAdded) {
      window.electron.watchFileOn(path, (_eventType: string, path: Path) => callback(path))
    }
    setDependencyArrayTracked(pathsRemaining.concat(pathsAdded))
  }, dependencyArray)
}
