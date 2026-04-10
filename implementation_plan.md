# Implementation Plan: Full CurseForge Parity with Prism Launcher

The goal is to align the Nebula Launcher's CurseForge integration with the robust standards set by Prism Launcher, specifically handling restricted mods and fine-tuning metadata mapping.

## Proposed Changes

### 1. Networking Layer Polish
Align headers with Prism's proven configuration.

#### [MODIFY] [util/fetch.rs](file:///c:/Users/minec/Documents/GitHub/code/packages/app-lib/src/util/fetch.rs)
- Ensure the `x-api-key` is included in ALL CurseForge requests, not just JSON calls.
- Standardize the `User-Agent` and `Accept` headers to match Prism's community identity.

### 2. Native Modpack Engine: Edge Fallback
Implement the "hidden" download mechanism Prism uses for mods that have "Third Party Distribution" disabled.

#### [MODIFY] [api/pack/install_curseforge.rs](file:///c:/Users/minec/Documents/GitHub/code/packages/app-lib/src/api/pack/install_curseforge.rs)
- Implement `get_edge_url(file_id, file_name)` helper.
- Update the download loop to check for `download_url == null`.
- If null, generate and use the `edge.forgecdn.net` fallback URL.
- This ensures that popular packs (like RLCraft) install correctly even if authors restrict API downloads.

### 3. Metadata & UI Synchronization
Ensure the discovery view provides the same amount of information as Prism.

#### [MODIFY] [api/curseforge.rs](file:///c:/Users/minec/Documents/GitHub/code/packages/app-lib/src/api/curseforge.rs)
- Add `is_available` and `expose_as_curse_mod` to the data structures.
- Update search and detail fetching to include these flags.

#### [MODIFY] [apps/app-frontend/src/pages/project/Index.vue](file:///c:/Users/minec/Documents/GitHub/code/apps/app-frontend/src/pages/project/Index.vue)
- Map the `is_available` flag to the UI.
- Show a warning if a mod is only available via the fallback mechanism (Edge).

## Verification Plan

### Automated Tests
- Test installation of a modpack known to have restricted mods (e.g., RLCraft or Pixelmon).
- Verify that the Edge fallback is triggered when the API returns no download URL.

### Manual Verification
- Check the project detail page for a restricted mod and ensure it still shows an "Install" button with a fallback warning.
- Verify that search results match Prism's "Featured" and "Popularity" sorting exactly.
