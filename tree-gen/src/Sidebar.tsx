import { createSignal, For, onCleanup, Show } from 'solid-js'
import type { SetStoreFunction } from 'solid-js/store'
import type * as THREE from 'three'
import { NumberInput } from '@ark-ui/solid/number-input'
import { Slider } from '@ark-ui/solid/slider'
import { downloadBlob, geometryToStl } from './export/stl'
import { meshToGeometry } from './mesh/toGeometry'
import type { Placement } from './placement'
import type { ReferenceModel } from './reference/reference'
import { createMeshWorker } from './tree/meshWorker'
import { treeParamGroups, type TreeParams } from './tree/params'

interface SidebarProps {
  params: TreeParams
  setParams: SetStoreFunction<TreeParams>
  placement: Placement
  setPlacement: SetStoreFunction<Placement>
  /** The tree-to-CAD transform the STL is exported with. */
  placementTransform: THREE.Matrix4
  reference: ReferenceModel | null
  onImport: (file: File) => void
  onRemoveReference: () => void
  importing: boolean
  importError: string | null
}

export function Sidebar(props: SidebarProps) {
  return (
    <div class="sidebar-content">
      <h1 class="sidebar-title">Tree Gen</h1>
      <For each={treeParamGroups}>
        {(group) => (
          <section class="param-group">
            <h2 class="param-group-title">{group.title}</h2>
            <For each={group.params}>
              {(spec) => (
                <ParamSlider
                  {...spec}
                  value={props.params[spec.key]}
                  onChange={(value) => props.setParams(spec.key, value)}
                />
              )}
            </For>
          </section>
        )}
      </For>
      <PlacementSection placement={props.placement} setPlacement={props.setPlacement} />
      <ReferenceSection
        reference={props.reference}
        onImport={props.onImport}
        onRemove={props.onRemoveReference}
        importing={props.importing}
        error={props.importError}
      />
      <ExportSection params={props.params} placement={props.placement} transform={props.placementTransform} />
    </div>
  )
}

function PlacementSection(props: { placement: Placement; setPlacement: SetStoreFunction<Placement> }) {
  return (
    <section class="param-group">
      <h2 class="param-group-title">Placement</h2>
      <NumberField
        label="Trunk height (mm)"
        title="Real height of the trunk from the ground to the top cut. Sets the size of the export."
        min={1}
        value={props.placement.heightMm}
        onChange={(value) => props.setPlacement('heightMm', value)}
      />
      <ParamSlider
        label="Rotation"
        min={-180}
        max={180}
        step={1}
        unit="°"
        value={props.placement.rotation}
        onChange={(value) => props.setPlacement('rotation', value)}
      />
      <p class="hint">Position of the trunk's base center, in CAD coordinates (mm, Z up).</p>
      <div class="field-row">
        <For each={['x', 'y', 'z'] as const}>
          {(axis) => (
            <NumberField
              label={axis.toUpperCase()}
              value={props.placement[axis]}
              onChange={(value) => props.setPlacement(axis, value)}
            />
          )}
        </For>
      </div>
    </section>
  )
}

interface ReferenceSectionProps {
  reference: ReferenceModel | null
  onImport: (file: File) => void
  onRemove: () => void
  importing: boolean
  error: string | null
}

function ReferenceSection(props: ReferenceSectionProps) {
  let fileInput!: HTMLInputElement

  return (
    <section class="param-group">
      <h2 class="param-group-title">Reference</h2>
      <input
        ref={fileInput}
        type="file"
        accept=".step,.stp"
        hidden
        onChange={(event) => {
          const file = event.currentTarget.files?.[0]
          // Clear so picking the same file again (e.g. after re-exporting it) still loads it.
          event.currentTarget.value = ''
          if (file) props.onImport(file)
        }}
      />
      <Show when={props.reference}>
        {(reference) => (
          <div class="reference-file">
            <span class="reference-name" title={reference().name}>
              {reference().name}
            </span>
            <button type="button" class="link-button" onClick={() => props.onRemove()}>
              Remove
            </button>
          </div>
        )}
      </Show>
      <button type="button" class="button secondary" disabled={props.importing} onClick={() => fileInput.click()}>
        {props.importing ? 'Importing…' : props.reference ? 'Replace STEP…' : 'Import STEP…'}
      </button>
      <Show when={props.error}>
        <p class="error">Import failed: {props.error}</p>
      </Show>
    </section>
  )
}

interface ExportSectionProps {
  params: TreeParams
  placement: Placement
  transform: THREE.Matrix4
}

function ExportSection(props: ExportSectionProps) {
  const [detail, setDetail] = createSignal(160)
  const [exporting, setExporting] = createSignal(false)
  const meshWorker = createMeshWorker()
  onCleanup(() => meshWorker.dispose())

  const exportStl = async () => {
    setExporting(true)
    // Read the placement now, so later edits during generation don't change this export.
    const transform = props.transform.clone()
    const heightMm = props.placement.heightMm
    try {
      const mesh = await meshWorker.generate(props.params, detail())
      const geometry = meshToGeometry(mesh)
      const blob = geometryToStl(geometry, transform)
      geometry.dispose()
      downloadBlob(blob, `oak-seed${props.params.seed}-${heightMm}mm.stl`)
    } finally {
      setExporting(false)
    }
  }

  return (
    <section class="param-group">
      <h2 class="param-group-title">Export</h2>
      <ParamSlider label="Detail" min={64} max={256} step={8} value={detail()} onChange={setDetail} />
      <button type="button" class="button" disabled={exporting()} onClick={exportStl}>
        {exporting() ? 'Exporting…' : 'Export STL'}
      </button>
    </section>
  )
}

interface NumberFieldProps {
  label: string
  title?: string
  min?: number
  value: number
  onChange: (value: number) => void
}

function NumberField(props: NumberFieldProps) {
  return (
    <NumberInput.Root
      class="param"
      min={props.min}
      step={1}
      value={String(props.value)}
      onValueChange={(details) => {
        const value = details.valueAsNumber
        if (Number.isFinite(value) && value >= (props.min ?? -Infinity)) props.onChange(value)
      }}
    >
      <NumberInput.Label class="field-label" title={props.title}>{props.label}</NumberInput.Label>
      <NumberInput.Control class="number-control">
        <NumberInput.Input class="number-input" />
      </NumberInput.Control>
    </NumberInput.Root>
  )
}

interface ParamSliderProps {
  label: string
  min: number
  max: number
  step: number
  unit?: string
  value: number
  onChange: (value: number) => void
}

function ParamSlider(props: ParamSliderProps) {
  const decimals = () => (props.step.toString().split('.')[1] ?? '').length

  return (
    <Slider.Root
      class="param"
      min={props.min}
      max={props.max}
      step={props.step}
      value={[props.value]}
      onValueChange={(details) => props.onChange(details.value[0])}
    >
      <div class="param-header">
        <Slider.Label class="param-label">{props.label}</Slider.Label>
        <span class="param-value">
          {props.value.toFixed(decimals())}
          {props.unit ? ` ${props.unit}` : ''}
        </span>
      </div>
      <Slider.Control class="slider-control">
        <Slider.Track class="slider-track">
          <Slider.Range class="slider-range" />
        </Slider.Track>
        <Slider.Thumb index={0} class="slider-thumb">
          <Slider.HiddenInput />
        </Slider.Thumb>
      </Slider.Control>
    </Slider.Root>
  )
}
