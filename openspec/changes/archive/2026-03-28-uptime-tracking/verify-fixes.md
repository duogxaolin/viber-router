## [2026-03-28] Round 1 (from spx-apply auto-verify)

### spx-uiux-verifier
- Fixed: Added loading skeleton state for uptime bars in GroupDetailPage servers tab (was blank during fetch)
- Fixed: Made auto-refresh in PublicUsagePage call fetchUptime independently so uptime refreshes even if usage fetch fails
- Fixed: UptimeBars accessibility — removed role="img" from container, added role="group" with aria-label, gave each bar role="img" + tabindex="0" + focus/blur handlers for keyboard access
- Fixed: Added prefers-reduced-motion media query to disable bar hover transition for users with motion sensitivity
- Fixed: Replaced hardcoded font-size: 12px in tooltip with text-caption class for design consistency
