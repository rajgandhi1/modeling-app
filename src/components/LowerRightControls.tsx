import { CoreDumpManager } from 'lib/coredump'

export function LowerRightControls({
  children,
}: {
  children?: React.ReactNode
  coreDumpManager?: CoreDumpManager
}) {
  return (
    <section className="absolute bottom-2 right-2 flex flex-col items-end gap-3 pointer-events-none">
      {children}
    </section>
  )
}
