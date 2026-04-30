# K2.16 holdback register

| Cluster | Why held back | Future evidence required | Schema-blocking | Descriptor-blocking | Backend-blocking |
|---|---|---|---:|---:|---:|
| `shadows/*` | shadow composition needs backend/subcell rendering semantics | backend adapter prototype plus visual/oracle review | no | partly | yes |
| `subcell_shapes/*` | requires subcell renderer and precise glyph/raster strategy | backend subcell proof and descriptor pass | no | partly | yes |
| shadow/subcell mixes | combines both backend holdbacks | compositor-backed player backend evidence | no | partly | yes |
| GUI conflict fixtures | require human visual/interaction judgment | GUI review lane and screenshots | no | no | partly |
| oracle/capture records | offline capture evidence, not runtime behavior | owner decision to keep as oracle or compile-time artifact | no | no | no |
| duplicate variants | redundant legacy recipes after canonical fixture exists | no implementation; keep mapping note | no | no | no |
| deprecated legacy records | historical evidence only | no implementation unless owner reactivates | no | no | no |

Holdbacks remain non-schema blockers under the accepted schema-readiness disposition model.
