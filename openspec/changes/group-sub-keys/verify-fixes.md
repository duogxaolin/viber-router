## [2026-03-26] Round 1 (from spx-apply auto-verify)

### spx-verifier
- Fixed: Removed 4 leftover placeholder comments from group_keys.rs and GroupDetailPage.vue
- Fixed: Invalid UUID in group_key_id filter now returns HTTP 400 instead of silently falling back to IS NULL (token_usage.rs)

### spx-arch-verifier
- Fixed: regenerate_key in groups.rs now calls invalidate_group_all_keys to invalidate sub-key cache entries on master key regeneration

### spx-uiux-verifier
- Fixed: Added aria-label to sub-key toggle and copy button in Keys tab
- Fixed: Added keyboard accessibility (tabindex, role, aria-expanded, keydown.enter) to expandable table rows
- Fixed: Added error state with retry button for sub-keys load failure
- Fixed: Added @hide handler to create key dialog to reset name on cancel
