# seiscache

`seiscache` is the cache, residency, and data-movement layer for the TraceBoost architecture.

This repo exists to implement Layer 3 from the `seismic_wgpu_architecture.md` plan:

- hot cache of recently used chunks
- preview cache for processed 2D sections
- GPU staging and upload preparation
- eviction policy and residency management
- optional multiresolution tiles for interactive viewing

## Why this repo exists

The current stack has ingest and storage primitives, but it does not yet have the layer that turns chunked data into responsive interactive behavior. Without that layer, the system will keep paying for repeated reads, repeated decompression, repeated conversions, and repeated upload work.

`seiscache` is meant to isolate those concerns from:

- `sgyx`, which should stay focused on SEG-Y I/O
- `seisrefine`, which should stay focused on storage and compute kernels
- `TraceBoost`, which should stay focused on app workflow and UI

## Intended responsibilities

- CPU-side chunk cache
- preview result cache keyed by dataset, viewport, and processing parameters
- conversion of stored chunks into contiguous `f32` buffers ready for compute
- GPU upload/staging abstractions used by backend `wgpu` compute
- cache policy, sizing, and eviction metrics
- multiresolution section tile residency where justified by profiling

## Intended non-goals

- raw SEG-Y parsing
- persistent canonical volume storage format definition
- seismic processing algorithms themselves
- frontend rendering code
- end-user job orchestration UI

## Relationship to sibling repos

- `seisdomain`: provides the stable chunk, section, tile, and preview contract types
- `sgyx`: supplies source reads when the working store or cache misses
- `seisrefine`: supplies chunked working-store access and processing kernels
- `seisjobs`: asks this layer for residency-aware preview and batch inputs
- `TraceBoost`: consumes cache-backed commands through the Tauri bridge

## Initial package direction

Likely Rust crates:

- `seiscache-core`
- `seiscache-preview`
- `seiscache-gpu`

## First deliverables

- LRU chunk cache over the working store
- preview cache keyed by section ROI and processing params
- chunk-to-contiguous-buffer path for compute
- instrumentation for hit/miss, bytes moved, and upload counts
- interfaces that can later back `wgpu` staging without forcing UI code into the backend
