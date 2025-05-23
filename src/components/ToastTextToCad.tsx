import type {
  TextToCad_type,
  TextToCadMultiFileIteration_type,
} from '@kittycad/lib/dist/types/src/models'
import { useCallback, useEffect, useRef } from 'react'
import toast from 'react-hot-toast'
import type { Mesh } from 'three'
import {
  Box3,
  Color,
  DirectionalLight,
  EdgesGeometry,
  LineBasicMaterial,
  LineSegments,
  MeshBasicMaterial,
  OrthographicCamera,
  Scene,
  Vector3,
  WebGLRenderer,
} from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'
import { DRACOLoader } from 'three/examples/jsm/loaders/DRACOLoader'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import { ActionButton } from '@src/components/ActionButton'
import { base64Decode } from '@src/lang/wasm'
import { isDesktop } from '@src/lib/isDesktop'
import { openExternalBrowserIfDesktop } from '@src/lib/openWindow'
import { PATHS } from '@src/lib/paths'
import { codeManager, kclManager, systemIOActor } from '@src/lib/singletons'
import { sendTelemetry } from '@src/lib/textToCadTelemetry'
import { reportRejection } from '@src/lib/trap'
import {
  SystemIOMachineEvents,
  waitForIdleState,
} from '@src/machines/systemIO/utils'
import {
  useProjectDirectoryPath,
  useRequestedFileName,
  useRequestedProjectName,
} from '@src/machines/systemIO/hooks'
import { commandBarActor } from '@src/lib/singletons'
import type { FileMeta } from '@src/lib/types'
import type { RequestedKCLFile } from '@src/machines/systemIO/utils'
import {
  Marked,
  type MarkedOptions,
  escape,
  unescape,
} from '@ts-stack/markdown'
import { SafeRenderer } from '@src/lib/markdown'

const CANVAS_SIZE = 128
const PROMPT_TRUNCATE_LENGTH = 128
const FRUSTUM_SIZE = 0.5
const OUTPUT_KEY = 'source.glb'

function TextToCadImprovementMessage({
  label,
  ...rest
}: React.HTMLAttributes<HTMLDetailsElement> & { label: string }) {
  return (
    <details {...rest}>
      <summary className="text-chalkboard-70 dark:text-chalkboard-30">
        {label}
      </summary>
      <p className="text-sm text-chalkboard-70 dark:text-chalkboard-30">
        Text-to-CAD is a new ML model. There will be prompts that work and
        prompts that don't and prompts that generate something a little bit off.
        Sometimes even a small tweak to your prompt will make it better on the
        next run. Try our prompt-to-edit feature to iterate on your result with
        AI. We look at all the failures to make the model better and see our
        weaknesses. Over time the model will get better. See our{' '}
        <a
          href="https://discord.gg/JQEpHR7Nt2"
          onClick={openExternalBrowserIfDesktop(
            'https://discord.gg/JQEpHR7Nt2'
          )}
        >
          Discord
        </a>{' '}
        or{' '}
        <a
          href="https://community.zoo.dev/"
          onClick={openExternalBrowserIfDesktop('https://community.zoo.dev/')}
        >
          Discourse
        </a>{' '}
        or some prompting tips from the community or our team.
      </p>
    </details>
  )
}

export function ToastTextToCadError({
  toastId,
  message,
  prompt,
  method,
  projectName,
  newProjectName,
}: {
  toastId: string
  message: string
  prompt: string
  method: string
  projectName: string
  newProjectName: string
}) {
  const markedOptions: MarkedOptions = {
    gfm: true,
    breaks: true,
    sanitize: true,
    unescape,
    escape,
  }
  return (
    <div className="flex flex-col justify-between gap-6">
      <section>
        <h2>Text-to-CAD failed</h2>
        <div
          className="parsed-markdown mt-4 text-sm text-chalkboard-70 dark:text-chalkboard-30 max-h-60 overflow-y-auto whitespace-normal"
          dangerouslySetInnerHTML={{
            __html: Marked.parse(message, {
              renderer: new SafeRenderer(markedOptions),
              ...markedOptions,
            }),
          }}
        />
      </section>
      <div className="flex justify-between gap-8">
        <ActionButton
          Element="button"
          iconStart={{
            icon: 'close',
          }}
          name="Dismiss"
          onClick={() => {
            toast.dismiss(toastId)
          }}
        >
          Dismiss
        </ActionButton>
        <ActionButton
          Element="button"
          iconStart={{
            icon: 'refresh',
          }}
          name="Edit prompt"
          onClick={() => {
            commandBarActor.send({
              type: 'Find and select command',
              data: {
                groupId: 'application',
                name: 'Text-to-CAD',
                argDefaultValues: {
                  prompt,
                  method,
                  projectName,
                  newProjectName,
                },
              },
            })
            toast.dismiss(toastId)
          }}
        >
          Edit prompt
        </ActionButton>
      </div>
    </div>
  )
}

export function ToastTextToCadSuccess({
  toastId,
  data,
  navigate,
  token,
  settings,
  projectName,
  fileName,
  isProjectNew,
  rootProjectName,
}: {
  toastId: string
  data: TextToCad_type & { fileName: string }
  navigate: (to: string) => void
  token?: string
  settings?: {
    highlightEdges: boolean
  }
  projectName: string
  fileName: string
  isProjectNew: boolean
  rootProjectName: string
}) {
  const wrapperRef = useRef<HTMLDivElement | null>(null)
  const canvasRef = useRef<HTMLCanvasElement | null>(null)
  const animationRequestRef = useRef<number>()
  const modelId = data.id
  const projectDirectoryPath = useProjectDirectoryPath()

  const animate = useCallback(
    ({
      renderer,
      scene,
      camera,
      controls,
      isFirstRender = false,
    }: {
      renderer: WebGLRenderer
      scene: Scene
      camera: OrthographicCamera
      controls: OrbitControls
      isFirstRender?: boolean
    }) => {
      if (
        !wrapperRef.current ||
        !(isFirstRender || animationRequestRef.current)
      )
        return
      animationRequestRef.current = requestAnimationFrame(() =>
        animate({ renderer, scene, camera, controls })
      )
      // required if controls.enableDamping or controls.autoRotate are set to true
      controls.update()
      renderer.render(scene, camera)
    },
    []
  )

  useEffect(() => {
    if (!canvasRef.current) return

    const canvas = canvasRef.current
    const renderer = new WebGLRenderer({
      canvas,
      antialias: true,
      alpha: true,
    })
    renderer.setSize(CANVAS_SIZE, CANVAS_SIZE)
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))

    const scene = new Scene()
    const ambientLight = new DirectionalLight(new Color('white'), 8.0)
    scene.add(ambientLight)
    const camera = createCamera()
    const controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true
    const loader = new GLTFLoader()
    const dracoLoader = new DRACOLoader()
    dracoLoader.setDecoderPath('/examples/jsm/libs/draco/')
    loader.setDRACOLoader(dracoLoader)
    scene.add(camera)

    // Get the base64 encoded GLB file
    const buffer = base64Decode(data.outputs[OUTPUT_KEY])

    if (buffer instanceof Error) {
      toast.error('Error loading GLB file: ' + buffer.message)
      console.error('decoding buffer from base64 failed', buffer)
      return
    }

    loader.parse(
      buffer,
      '',
      // called when the resource is loaded
      function (gltf) {
        scene.add(gltf.scene)

        // Style the objects in the scene
        traverseSceneToStyleObjects({
          scene,
          ...settings,
        })

        // Then we'll calculate the max distance of the bounding box
        // and set that as the camera's position
        const size = new Vector3()
        const boundingBox = new Box3()
        boundingBox.setFromObject(gltf.scene)
        boundingBox.getSize(size)
        const maxDistance = Math.max(size.x, size.y, size.z)

        camera.position.set(maxDistance * 2, maxDistance * 2, maxDistance * 2)
        camera.lookAt(0, 0, 0)
        camera.left = -maxDistance
        camera.right = maxDistance
        camera.top = maxDistance
        camera.bottom = -maxDistance
        camera.near = 0
        camera.far = maxDistance * 10

        // Create and attach the lights,
        // since their position depends on the bounding box
        const cameraLight1 = new DirectionalLight(new Color('white'), 1)
        cameraLight1.position.set(maxDistance * -5, -maxDistance, maxDistance)
        camera.add(cameraLight1)

        const cameraLight2 = new DirectionalLight(new Color('white'), 1.4)
        cameraLight2.position.set(0, 0, 2 * maxDistance)
        camera.add(cameraLight2)

        const sceneLight = new DirectionalLight(new Color('white'), 1)
        sceneLight.position.set(
          -2 * maxDistance,
          -2 * maxDistance,
          2 * maxDistance
        )
        scene.add(sceneLight)

        camera.updateProjectionMatrix()
        controls.update()
        // render the scene once...
        renderer.render(scene, camera)
      },
      // called when loading has errors
      function (error) {
        toast.error('Error loading GLB file: ' + error.message)
        console.error('Error loading GLB file', error)
        return
      }
    )

    // ...and set a mouseover listener on the canvas to enable the orbit controls
    canvasRef.current.addEventListener('mouseover', () => {
      renderer.setAnimationLoop(() =>
        animate({ renderer, scene, camera, controls, isFirstRender: true })
      )
    })
    canvasRef.current.addEventListener('mouseout', () => {
      renderer.setAnimationLoop(null)
      if (animationRequestRef.current) {
        cancelAnimationFrame(animationRequestRef.current)
        animationRequestRef.current = undefined
      }
    })

    return () => {
      renderer.dispose()
      if (animationRequestRef.current) {
        cancelAnimationFrame(animationRequestRef.current)
        animationRequestRef.current = undefined
      }
    }
  }, [])

  return (
    <div className="flex gap-4 min-w-80" ref={wrapperRef}>
      <div
        className="flex-none overflow-hidden"
        style={{ width: CANVAS_SIZE + 'px', height: CANVAS_SIZE + 'px' }}
      >
        <canvas ref={canvasRef} width={CANVAS_SIZE} height={CANVAS_SIZE} />
      </div>
      <div className="flex flex-col justify-between gap-6">
        <section>
          <h2 className="font-sans font-bold">Text-to-CAD successful</h2>
          <p className="text-sm my-3">
            File "{fileName}" was successfully added to project "{projectName}"
            from prompt:
          </p>
          <blockquote className="my-3 border-solid border-l-2 pl-4 text-sm text-chalkboard-70 dark:text-chalkboard-30">
            {data.prompt.length > PROMPT_TRUNCATE_LENGTH
              ? data.prompt.slice(0, PROMPT_TRUNCATE_LENGTH) + '...'
              : data.prompt}
          </blockquote>
          <TextToCadImprovementMessage
            className="text-sm mt-2"
            label="Not what you expected?"
          />
        </section>
        <div className="flex justify-between gap-8">
          <ActionButton
            Element="button"
            iconStart={{
              icon: 'close',
            }}
            data-negative-button="reject"
            name="Reject"
            onClick={() => {
              sendTelemetry(modelId, 'rejected', token).catch(reportRejection)
              if (isDesktop()) {
                // Delete the file from the project

                if (isProjectNew) {
                  // Delete the entire project if it was newly created from text to CAD
                  systemIOActor.send({
                    type: SystemIOMachineEvents.deleteProject,
                    data: {
                      requestedProjectName: rootProjectName,
                    },
                  })
                } else if (projectName && fileName) {
                  // deletes the folder when inside the modeling page
                  // The TTC Create will make a subdir, delete that dir with the main.kcl as well
                  systemIOActor.send({
                    type: SystemIOMachineEvents.deleteProject,
                    data: {
                      requestedProjectName: projectName,
                    },
                  })
                }
              }

              toast.dismiss(toastId)
            }}
          >
            Reject
          </ActionButton>
          {isDesktop() ? (
            <ActionButton
              Element="button"
              iconStart={{
                icon: 'checkmark',
              }}
              name="Accept"
              onClick={() => {
                // eslint-disable-next-line @typescript-eslint/no-floating-promises
                sendTelemetry(modelId, 'accepted', token)
                const path = `${projectDirectoryPath}${window.electron.path.sep}${projectName}${window.electron.sep}${fileName}`
                navigate(`${PATHS.FILE}/${encodeURIComponent(path)}`)

                toast.dismiss(toastId)
              }}
            >
              Accept
            </ActionButton>
          ) : (
            <ActionButton
              Element="button"
              iconStart={{
                icon: 'checkmark',
              }}
              name="Replace current file"
              onClick={() => {
                // eslint-disable-next-line @typescript-eslint/no-floating-promises
                sendTelemetry(modelId, 'accepted', token)
                const code = data.code || '// no code found'

                systemIOActor.send({
                  type: SystemIOMachineEvents.createKCLFile,
                  data: {
                    requestedProjectName: projectName,
                    requestedCode: code,
                    requestedFileNameWithExtension: fileName,
                  },
                })

                toast.dismiss(toastId)
              }}
            >
              Replace current file
            </ActionButton>
          )}
        </div>
      </div>
    </div>
  )
}

const createCamera = (): OrthographicCamera => {
  return new OrthographicCamera(
    -FRUSTUM_SIZE,
    FRUSTUM_SIZE,
    FRUSTUM_SIZE,
    -FRUSTUM_SIZE,
    0.5,
    3
  )
}

function traverseSceneToStyleObjects({
  scene,
  color = 0x29ffa4,
  highlightEdges = false,
}: {
  scene: Scene
  color?: number
  highlightEdges?: boolean
}) {
  scene.traverse((child) => {
    if ('isMesh' in child && child.isMesh) {
      // Replace the material with our flat color one
      ;(child as Mesh).material = new MeshBasicMaterial({
        color,
      })

      // Add edges to the mesh if the option is enabled
      if (!highlightEdges) return
      const edges = new EdgesGeometry((child as Mesh).geometry, 30)
      const lines = new LineSegments(
        edges,
        new LineBasicMaterial({
          // We don't respect the theme here on purpose,
          // because I found the dark edges to work better
          // in light and dark themes,
          // but we can change that if needed, it is wired up
          color: 0x1f2020,
        })
      )
      scene.add(lines)
    }
  })
}

export function ToastPromptToEditCadSuccess({
  toastId,
  token,
  data,
  oldCodeWebAppOnly: oldCode,
  oldFiles,
}: {
  toastId: string
  oldCodeWebAppOnly: string
  oldFiles: FileMeta[]
  data: TextToCadMultiFileIteration_type
  token?: string
}) {
  const modelId = data.id
  const possibleRequestedProjectName = useRequestedProjectName()
  const possibleRequestedFileName = useRequestedFileName()

  // Depends on navigation method
  const requestedProjectName = {
    name:
      possibleRequestedProjectName.name || possibleRequestedFileName.project,
  }

  return (
    <div className="flex gap-4 min-w-80">
      <div className="flex flex-col justify-between gap-6">
        <section>
          <h2>Prompt to edit successful</h2>
          <p className="text-sm text-chalkboard-70 dark:text-chalkboard-30">
            Prompt: "
            {data?.prompt && data?.prompt?.length > PROMPT_TRUNCATE_LENGTH
              ? data.prompt.slice(0, PROMPT_TRUNCATE_LENGTH) + '...'
              : data.prompt}
            "
          </p>
          <TextToCadImprovementMessage
            className="text-sm mt-2"
            label="Not what you expected?"
          />
          <p>Do you want to keep the change?</p>
        </section>
        <div className="flex justify-between gap-8">
          <ActionButton
            Element="button"
            iconStart={{
              icon: 'close',
            }}
            data-negative-button={'reject'}
            name={'Reject'}
            onClick={() => {
              void (async () => {
                sendTelemetry(modelId, 'rejected', token).catch(reportRejection)
                // revert to before the prompt-to-edit
                if (isDesktop()) {
                  const requestedFiles: RequestedKCLFile[] = []
                  for (const file of oldFiles) {
                    if (file.type !== 'kcl') {
                      // only need to write the kcl files
                      // as the endpoint would have never overwritten other file types
                      continue
                    }
                    requestedFiles.push({
                      requestedCode: file.fileContents,
                      requestedFileName: file.relPath,
                      requestedProjectName: requestedProjectName.name,
                    })
                  }
                  toast.dismiss(toastId)
                  await writeOverFilesAndExecute({
                    requestedFiles: requestedFiles,
                    projectName: requestedProjectName.name,
                    filePath: possibleRequestedFileName.file,
                  })
                } else {
                  codeManager.updateCodeEditor(oldCode)
                  kclManager.executeCode().catch(reportRejection)
                  toast.dismiss(toastId)
                }
              })()
            }}
          >
            {'Revert'}
          </ActionButton>

          <ActionButton
            Element="button"
            iconStart={{
              icon: 'checkmark',
            }}
            name="Accept"
            onClick={() => {
              sendTelemetry(modelId, 'accepted', token).catch(reportRejection)
              toast.dismiss(toastId)
              /**
               * NO OP. Do not rewrite code to disk, we already do this ahead of time. This will dismiss the toast.
               * All of the files were already written! Don't rewrite the current code editor.
               * If this prompt to edit makes 5 new files, the code manager is only watching 1 of these files, why
               * would it rewrite the current file selected when this is completed?
               */
            }}
          >
            Continue
          </ActionButton>
        </div>
      </div>
    </div>
  )
}

export const writeOverFilesAndExecute = async ({
  requestedFiles,
  projectName,
  filePath,
}: {
  requestedFiles: RequestedKCLFile[]
  projectName: string
  filePath?: string | undefined
}) => {
  if (filePath) {
    systemIOActor.send({
      type: SystemIOMachineEvents.bulkCreateKCLFilesAndNavigateToFile,
      data: {
        files: requestedFiles,
        requestedProjectName: projectName,
        requestedFileNameWithExtension: filePath,
        override: true,
      },
    })
  } else {
    systemIOActor.send({
      type: SystemIOMachineEvents.bulkCreateKCLFilesAndNavigateToProject,
      data: {
        files: requestedFiles,
        requestedProjectName: projectName,
        override: true,
      },
    })
  }

  // to await the result of the send event above
  await waitForIdleState({ systemIOActor })
}
