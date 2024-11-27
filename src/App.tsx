import { useEffect, useMemo, useRef } from 'react'
import { useHotKeyListener } from './hooks/useHotKeyListener'
import { Stream } from './components/Stream'
import { AppHeader } from './components/AppHeader'
import { useHotkeys } from 'react-hotkeys-hook'
import { useLoaderData, useLocation, useNavigate } from 'react-router-dom'
import { type IndexLoaderData } from 'lib/types'
import { PATHS } from 'lib/paths'
import { useSettingsAuthContext } from 'hooks/useSettingsAuthContext'
import { onboardingPaths } from 'routes/Onboarding/paths'
import { useEngineConnectionSubscriptions } from 'hooks/useEngineConnectionSubscriptions'
import { codeManager, engineCommandManager } from 'lib/singletons'
import { useAbsoluteFilePath } from 'hooks/useAbsoluteFilePath'
import { isDesktop } from 'lib/isDesktop'
import { useLspContext } from 'components/LspProvider'
import { useRefreshSettings } from 'hooks/useRefreshSettings'
import { ModelingSidebar } from 'components/ModelingSidebar/ModelingSidebar'
import { LowerRightControls } from 'components/LowerRightControls'
import ModalContainer from 'react-modal-promise'
import useHotkeyWrapper from 'lib/hotkeyWrapper'
import Gizmo from 'components/Gizmo'
import { CoreDumpManager } from 'lib/coredump'
import { UnitsMenu } from 'components/UnitsMenu'
import { CameraProjectionToggle } from 'components/CameraProjectionToggle'
import { homeDefaultStatusBarItems } from 'components/statusBar/homeDefaultStatusBarItems'
import { StatusBar } from 'components/StatusBar'
import { useModelStateStatus } from 'components/ModelStateIndicator'
import { useNetworkHealthStatus } from 'components/NetworkHealthIndicator'
import { useModelingContext } from 'hooks/useModelingContext'
import { xStateValueToString } from 'lib/xStateValueToString'
import { maybeWriteToDisk } from 'lib/telemetry'
import { useNetworkMachineStatus } from 'components/NetworkMachineIndicator'
maybeWriteToDisk()
  .then(() => {})
  .catch(() => {})

export function App() {
  const { project, file } = useLoaderData() as IndexLoaderData
  useRefreshSettings(PATHS.FILE + 'SETTINGS')
  const navigate = useNavigate()
  const location = useLocation()
  const filePath = useAbsoluteFilePath()
  const { onProjectOpen } = useLspContext()
  const { state: modelingState, streamRef } = useModelingContext()

  const projectName = project?.name || null
  const projectPath = project?.path || null
  useEffect(() => {
    onProjectOpen({ name: projectName, path: projectPath }, file || null)
  }, [projectName, projectPath])

  useHotKeyListener()

  const { auth, settings } = useSettingsAuthContext()
  const token = auth?.context?.token

  const coreDumpManager = useMemo(
    () => new CoreDumpManager(engineCommandManager, codeManager, token),
    []
  )

  const {
    app: { onboardingStatus },
  } = settings.context

  useHotkeys('backspace', (e) => {
    e.preventDefault()
  })
  useHotkeyWrapper(
    [isDesktop() ? 'mod + ,' : 'shift + mod + ,'],
    () => navigate(filePath + PATHS.SETTINGS),
    {
      splitKey: '|',
    }
  )

  const paneOpacity = [onboardingPaths.CAMERA, onboardingPaths.STREAMING].some(
    (p) => p === onboardingStatus.current
  )
    ? 'opacity-20'
    : ''

  useEngineConnectionSubscriptions()

  return (
    <div className="h-screen flex flex-col overflow-hidden select-none">
      <div className="relative flex flex-1 flex-col" ref={streamRef}>
        <AppHeader
          className={'transition-opacity transition-duration-75 ' + paneOpacity}
          project={{ project, file }}
          enableMenu={true}
        />
        <ModalContainer />
        <ModelingSidebar paneOpacity={paneOpacity} />
        <Stream />
        {/* <CamToggle /> */}
        <LowerRightControls coreDumpManager={coreDumpManager}>
          <UnitsMenu />
          <Gizmo />
          <CameraProjectionToggle />
        </LowerRightControls>
      </div>
      <StatusBar
        globalItems={[
          useNetworkHealthStatus(),
          useNetworkMachineStatus(),
          ...homeDefaultStatusBarItems({ coreDumpManager, location }),
        ]}
        localItems={[
          {
            id: 'modeling-state',
            element: 'text',
            label:
              modelingState.value instanceof Object
                ? xStateValueToString(modelingState.value) ?? ''
                : modelingState.value,
            toolTip: {
              children: 'The current state of the modeler',
            },
          },
          useModelStateStatus(),
        ]}
      />
    </div>
  )
}
