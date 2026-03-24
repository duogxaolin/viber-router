<template>
  <q-page padding>
    <div class="text-h5 q-mb-md">Proxy Error Logs</div>

    <div class="row q-col-gutter-sm q-mb-md">
      <div class="col-auto">
        <q-select
          v-model="filters.status_code"
          :options="statusOptions"
          label="Status Code"
          outlined dense clearable emit-value map-options
          style="min-width: 140px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-select
          v-model="filters.group_id"
          :options="groupOptions"
          label="Group"
          outlined dense clearable emit-value map-options
          style="min-width: 180px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-select
          v-model="filters.server_id"
          :options="serverOptions"
          label="Server"
          outlined dense clearable emit-value map-options
          style="min-width: 180px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-select
          v-model="filters.error_type"
          :options="errorTypeOptions"
          label="Error Type"
          outlined dense clearable emit-value map-options
          style="min-width: 180px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-input
          v-model="filters.from"
          label="From"
          outlined dense type="datetime-local"
          style="min-width: 200px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-input
          v-model="filters.to"
          label="To"
          outlined dense type="datetime-local"
          style="min-width: 200px"
          @update:model-value="onFilterChange"
        />
      </div>
      <div class="col-auto">
        <q-input
          v-model="apiKeySearch"
          label="API Key"
          outlined dense
          style="min-width: 200px"
          @keyup.enter="onApiKeySearch"
        >
          <template #append>
            <q-icon name="search" class="cursor-pointer" @click="onApiKeySearch" />
          </template>
        </q-input>
      </div>
    </div>

    <q-banner v-if="error" class="bg-negative text-white q-mb-md">
      Failed to load logs
      <template #action>
        <q-btn flat label="Retry" @click="() => fetchLogs()" />
      </template>
    </q-banner>

    <q-table
      :rows="logs"
      :columns="columns"
      row-key="id"
      :loading="loading"
      flat bordered
      hide-pagination
    >
      <template #body="props">
        <q-tr :props="props" class="cursor-pointer" @click="props.expand = !props.expand">
          <q-td v-for="col in props.cols" :key="col.name" :props="props">
            <template v-if="col.name === 'status_code'">
              <q-badge :color="statusColor(props.row.status_code)" :label="String(props.row.status_code)" />
            </template>
            <template v-else-if="col.name === 'error_type'">
              <q-badge
                :color="errorTypeBadge(props.row.error_type).color"
                :label="errorTypeBadge(props.row.error_type).label"
                outline
              />
            </template>
            <template v-else-if="col.name === 'server_name'">
              <span v-if="props.row.failover_chain.length > 1" class="server-chain">
                <template v-for="(attempt, i) in props.row.failover_chain" :key="i">
                  <span :class="attempt.status >= 200 && attempt.status < 400 ? 'text-positive' : 'text-negative'">{{ attempt.server_name }}</span>
                  <q-icon v-if="Number(i) < props.row.failover_chain.length - 1" name="arrow_forward" size="xs" class="q-mx-xs text-grey" />
                </template>
              </span>
              <span v-else>{{ props.row.server_name }}</span>
            </template>
            <template v-else-if="col.name === 'latency_ms'">
              {{ props.row.latency_ms }}ms
            </template>
            <template v-else-if="col.name === 'created_at'">
              {{ formatDate(props.row.created_at) }}
            </template>
            <template v-else>
              {{ col.value }}
            </template>
          </q-td>
        </q-tr>
        <q-tr v-show="props.expand" :props="props">
          <q-td colspan="100%">
            <div class="q-pa-md">
              <div class="row q-col-gutter-md">
                <div class="col-12 col-md-6">
                  <div class="text-subtitle2 q-mb-sm">Request Details</div>
                  <div><span class="text-weight-medium">Path:</span> {{ props.row.request_method }} {{ props.row.request_path }}</div>
                  <div><span class="text-weight-medium">API Key:</span> <code>{{ props.row.group_api_key }}</code></div>
                  <div><span class="text-weight-medium">Model:</span> {{ props.row.request_model || 'N/A' }}</div>
                  <div><span class="text-weight-medium">Error Type:</span> {{ props.row.error_type }}</div>
                  <q-btn
                    v-if="props.row.upstream_url"
                    flat dense no-caps
                    icon="download"
                    label="Download cURL"
                    color="primary"
                    class="q-mt-sm"
                    @click.stop="downloadCurl(props.row)"
                  />
                </div>
                <div class="col-12 col-md-6">
                  <div class="text-subtitle2 q-mb-sm">Failover Chain</div>
                  <div v-if="props.row.failover_chain.length === 0" class="text-grey">No failover data</div>
                  <div v-else class="failover-timeline">
                    <div v-for="(attempt, i) in props.row.failover_chain" :key="i" class="failover-step">
                      <div class="failover-dot" :class="attemptClass(attempt)" />
                      <div class="failover-line" v-if="Number(i) < props.row.failover_chain.length - 1" />
                      <div class="failover-info">
                        <div class="text-weight-medium">
                          {{ attempt.server_name }}
                          <q-badge
                            :color="attempt.status === 0 ? 'grey' : attempt.status >= 200 && attempt.status < 400 ? 'positive' : 'negative'"
                            :label="attempt.status === 0 ? 'ERR' : String(attempt.status)"
                            class="q-ml-sm"
                          />
                          <q-icon v-if="attempt.status >= 200 && attempt.status < 400" name="check_circle" color="positive" size="xs" class="q-ml-xs" />
                        </div>
                        <div class="text-caption text-grey">{{ attempt.latency_ms }}ms</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </q-td>
        </q-tr>
      </template>

      <template #no-data>
        <div class="full-width text-center q-pa-lg text-grey">
          No logs matching filters. Try adjusting your filter criteria.
        </div>
      </template>
    </q-table>

    <div class="row justify-center q-mt-md" v-if="nextCursor">
      <q-btn flat label="Load More" @click="loadMore" :loading="loading" />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue';
import { api } from 'boot/axios';

interface FailoverAttempt {
  server_id: string;
  server_name: string;
  status: number;
  latency_ms: number;
}

interface ProxyLog {
  id: string;
  created_at: string;
  group_id: string;
  group_api_key: string;
  server_id: string;
  server_name: string;
  request_path: string;
  request_method: string;
  status_code: number;
  error_type: string;
  latency_ms: number;
  failover_chain: FailoverAttempt[];
  request_model: string | null;
  request_body: Record<string, unknown> | null;
  request_headers: Record<string, string> | null;
  upstream_url: string | null;
}

interface LogListResponse {
  data: ProxyLog[];
  next_cursor: string | null;
}

const logs = ref<ProxyLog[]>([]);
const loading = ref(false);
const error = ref(false);
const nextCursor = ref<string | null>(null);
const apiKeySearch = ref('');

const filters = reactive({
  status_code: null as number | null,
  group_id: null as string | null,
  server_id: null as string | null,
  error_type: null as string | null,
  from: null as string | null,
  to: null as string | null,
});

const groupOptions = ref<{ label: string; value: string }[]>([]);
const serverOptions = ref<{ label: string; value: string }[]>([]);

const statusOptions = [
  { label: '400', value: 400 },
  { label: '401', value: 401 },
  { label: '403', value: 403 },
  { label: '429', value: 429 },
  { label: '500', value: 500 },
  { label: '502', value: 502 },
  { label: '503', value: 503 },
];

const errorTypeOptions = [
  { label: 'Upstream Error', value: 'upstream_error' },
  { label: 'Failover Success', value: 'failover_success' },
  { label: 'All Servers Exhausted', value: 'all_servers_exhausted' },
  { label: 'Connection Error', value: 'connection_error' },
];

const columns = [
  { name: 'created_at', label: 'Time', field: 'created_at', align: 'left' as const },
  { name: 'status_code', label: 'Status', field: 'status_code', align: 'left' as const },
  { name: 'error_type', label: 'Type', field: 'error_type', align: 'left' as const },
  { name: 'server_name', label: 'Server', field: (row: ProxyLog) => {
    if (row.failover_chain.length > 1) {
      return row.failover_chain.map(a => a.server_name).join(' → ');
    }
    return row.server_name;
  }, align: 'left' as const },
  { name: 'request_model', label: 'Model', field: (row: ProxyLog) => row.request_model || '-', align: 'left' as const },
  { name: 'latency_ms', label: 'Latency', field: 'latency_ms', align: 'right' as const },
];

function statusColor(code: number): string {
  if (code >= 500) return 'negative';
  if (code >= 400) return 'warning';
  return 'positive';
}

function errorTypeBadge(type: string): { color: string; label: string } {
  switch (type) {
    case 'failover_success': return { color: 'amber', label: 'Failover ✓' };
    case 'all_servers_exhausted': return { color: 'negative', label: 'All Failed' };
    case 'connection_error': return { color: 'grey', label: 'Conn Error' };
    default: return { color: 'warning', label: 'Upstream Error' };
  }
}

function attemptClass(attempt: FailoverAttempt): string {
  if (attempt.status === 0) return 'bg-grey';
  if (attempt.status >= 200 && attempt.status < 400) return 'bg-positive';
  return 'bg-negative';
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleString();
}

function buildParams(cursor?: string | null) {
  const params: Record<string, string | number> = {};
  if (filters.status_code) params.status_code = filters.status_code;
  if (filters.group_id) params.group_id = filters.group_id;
  if (filters.server_id) params.server_id = filters.server_id;
  if (filters.error_type) params.error_type = filters.error_type;
  if (filters.from) params.from = new Date(filters.from).toISOString();
  if (filters.to) params.to = new Date(filters.to).toISOString();
  if (apiKeySearch.value) params.api_key = apiKeySearch.value;
  if (cursor) params.cursor = cursor;
  params.page_size = 20;
  return params;
}

async function fetchLogs(cursor?: string | null) {
  loading.value = true;
  error.value = false;
  try {
    const { data } = await api.get<LogListResponse>('/api/admin/logs', {
      params: buildParams(cursor),
    });
    if (cursor) {
      logs.value.push(...data.data);
    } else {
      logs.value = data.data;
    }
    nextCursor.value = data.next_cursor;
  } catch {
    error.value = true;
  } finally {
    loading.value = false;
  }
}

function onFilterChange() {
  nextCursor.value = null;
  fetchLogs();
}

function onApiKeySearch() {
  nextCursor.value = null;
  fetchLogs();
}

function loadMore() {
  if (nextCursor.value) {
    fetchLogs(nextCursor.value);
  }
}

function shellEscape(s: string): string {
  return `'${s.replace(/'/g, "'\\''")}'`;
}

function generateCurl(log: ProxyLog): string {
  const parts: string[] = ['curl'];

  if (log.request_method !== 'GET') {
    parts.push(`-X ${log.request_method}`);
  }

  parts.push(shellEscape(log.upstream_url ?? ''));

  if (log.request_headers) {
    for (const [name, value] of Object.entries(log.request_headers)) {
      parts.push(`-H ${shellEscape(`${name}: ${value}`)}`);
    }
  }

  if (log.request_body) {
    parts.push(`-d ${shellEscape(JSON.stringify(log.request_body))}`);
  }

  return parts.join(' \\\n  ');
}

function downloadCurl(log: ProxyLog) {
  const curl = generateCurl(log);
  const blob = new Blob([`#!/bin/bash\n${curl}\n`], { type: 'text/x-shellscript' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `curl-${log.id.slice(0, 8)}.sh`;
  a.click();
  URL.revokeObjectURL(url);
}

async function loadFilterOptions() {
  try {
    const [groups, servers] = await Promise.all([
      api.get('/api/admin/groups', { params: { limit: 100 } }),
      api.get('/api/admin/servers', { params: { limit: 100 } }),
    ]);
    groupOptions.value = groups.data.data.map((g: { id: string; name: string }) => ({
      label: g.name,
      value: g.id,
    }));
    serverOptions.value = servers.data.data.map((s: { id: string; name: string }) => ({
      label: s.name,
      value: s.id,
    }));
  } catch {
    // Filter options are non-critical
  }
}

onMounted(() => {
  fetchLogs();
  loadFilterOptions();
});
</script>

<style scoped>
.failover-timeline {
  display: flex;
  flex-direction: column;
  gap: 0;
}
.failover-step {
  display: flex;
  align-items: flex-start;
  position: relative;
  padding-bottom: 8px;
}
.failover-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  margin-top: 5px;
  margin-right: 10px;
  flex-shrink: 0;
}
.failover-line {
  position: absolute;
  left: 4px;
  top: 15px;
  bottom: 0;
  width: 2px;
  background: #ccc;
}
.server-chain {
  display: inline-flex;
  align-items: center;
}
</style>
