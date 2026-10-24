import { createEffect, createMemo, createSignal, on } from 'solid-js'
import { createStore } from 'solid-js/store'
import { Splitter } from '@ark-ui/solid/splitter'
import { defaultPlacement, placementMatrix, type Placement } from './placement'
import { loadStepFile, type ReferenceModel } from './reference/reference'
import { Sidebar } from './Sidebar'
import { Viewport } from './Viewport'
import { defaultTreeParams, snapshotParams, type TreeParams } from './tree/params'
import { fitRoots } from './tree/rootPaths'
import { resampleSkeleton } from './tree/skeleton'

export function App() {
  const [params, setParams] = createStore<TreeParams>(snapshotParams(defaultTreeParams))
  const [placement, setPlacement] = createStore<Placement>({ ...defaultPlacement })
  const placementTransform = createMemo(() => placementMatrix({ ...placement }, params.height))

  const [reference, setReference] = createSignal<ReferenceModel | null>(null)
  const [importing, setImporting] = createSignal<string | null>(null)
  const [importError, setImportError] = createSignal<string | null>(null)
  const importReference = async (file: File) => {
    setImporting(file.name)
    setImportError(null)
    try {
      setReference(await loadStepFile(file))
    } catch (e) {
      setImportError(e instanceof Error ? e.message : String(e))
    } finally {
      setImporting(null)
    }
  }

  // Keep the skeleton as long as the control point count, reshaped to follow the same curve.
  createEffect(
    on(
      () => params.skeletonPoints,
      (count) => {
        if (params.skeleton.length !== count) setParams('skeleton', resampleSkeleton(params.skeleton, count))
      },
    ),
  )

  // Keep one root path per root, each with the set number of control points. New roots are generated;
  // existing ones keep their shape.
  createEffect(
    on(
      () => [params.rootCount, params.rootPoints],
      () => setParams('roots', fitRoots(snapshotParams(params))),
      { defer: true },
    ),
  )

  return (
    <Splitter.Root
      class="app"
      panels={[
        { id: 'sidebar', minSize: 10, maxSize: 50 },
        { id: 'main' },
      ]}
      defaultSize={[20, 80]}
    >
      <Splitter.Panel id="sidebar" class="sidebar">
        <Sidebar
          params={params}
          setParams={setParams}
          placement={placement}
          setPlacement={setPlacement}
          placementTransform={placementTransform()}
          reference={reference()}
          onImport={importReference}
          onRemoveReference={() => setReference(null)}
          importing={importing() !== null}
          importError={importError()}
        />
      </Splitter.Panel>
      <Splitter.ResizeTrigger id="sidebar:main" class="resize-trigger" aria-label="Resize sidebar" />
      <Splitter.Panel id="main">
        <Viewport
          params={params}
          reference={reference()}
          placement={placement}
          movePlacement={(change) => setPlacement(change)}
          importing={importing()}
          importError={importError()}
          edits={{
            moveTrunkPoint: (index, point) => setParams('skeleton', index, point),
            moveRootPoint: (root, index, point) => setParams('roots', root, 'points', index, point),
            setRootScale: (root, scale) => setParams('roots', root, 'scale', scale),
          }}
        />
      </Splitter.Panel>
    </Splitter.Root>
  )
}
