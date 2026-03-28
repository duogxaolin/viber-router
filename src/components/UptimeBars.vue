<template>
  <div class="uptime-bars">
    <div class="uptime-bars__container" role="group" aria-label="Uptime status bars">
      <div
        v-for="(b, i) in buckets"
        :key="i"
        class="uptime-bars__bar"
        role="img"
        tabindex="0"
        :style="{ backgroundColor: barColor(b) }"
        :aria-label="barLabel(b)"
        @mouseenter="hoveredIndex = i"
        @mouseleave="hoveredIndex = -1"
        @focus="hoveredIndex = i"
        @blur="hoveredIndex = -1"
      >
        <q-tooltip v-if="hoveredIndex === i" :offset="[0, 4]">
          <div class="text-caption">
            <div>{{ formatRange(b.timestamp) }}</div>
            <div v-if="b.total > 0">
              {{ b.success }}/{{ b.total }} ({{ pct(b) }}%)
            </div>
            <div v-else>No data</div>
          </div>
        </q-tooltip>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

export interface Bucket {
  timestamp: number;
  total: number;
  success: number;
}

defineProps<{
  buckets: Bucket[];
}>();

const hoveredIndex = ref(-1);

function pct(b: Bucket): string {
  if (b.total === 0) return '0';
  return ((b.success / b.total) * 100).toFixed(1);
}

function barColor(b: Bucket): string {
  if (b.total === 0) return 'var(--vr-uptime-gray, #9e9e9e)';
  const rate = b.success / b.total;
  if (rate > 0.95) return 'var(--vr-uptime-green, #4caf50)';
  if (rate >= 0.5) return 'var(--vr-uptime-yellow, #ff9800)';
  return 'var(--vr-uptime-red, #f44336)';
}

function barLabel(b: Bucket): string {
  const range = formatRange(b.timestamp);
  if (b.total === 0) return `${range}: No data`;
  return `${range}: ${pct(b)}% uptime, ${b.total} requests`;
}

function formatRange(ts: number): string {
  const start = new Date(ts * 1000);
  const end = new Date((ts + 1800) * 1000);
  const fmt = (d: Date) =>
    d.toLocaleString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  return `${fmt(start)} – ${fmt(end)}`;
}
</script>

<style scoped>
.uptime-bars__container {
  display: flex;
  gap: 1px;
  height: 24px;
  align-items: stretch;
}
.uptime-bars__bar {
  flex: 1;
  min-width: 2px;
  border-radius: 2px;
  cursor: pointer;
  transition: opacity 0.15s;
}
.uptime-bars__bar:hover,
.uptime-bars__bar:focus {
  opacity: 0.8;
  outline: none;
}
@media (prefers-reduced-motion: reduce) {
  .uptime-bars__bar {
    transition: none;
  }
}
</style>
