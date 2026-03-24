<template>
  <q-page padding>
    <div class="text-h5 q-mb-md">Settings</div>

    <q-card flat bordered style="max-width: 640px">
      <q-card-section>
        <div class="text-subtitle1 q-mb-md">Telegram Alerts</div>

        <q-input
          v-model="form.telegram_bot_token"
          label="Bot Token"
          outlined
          dense
          clearable
          class="q-mb-sm"
        />

        <div class="q-mb-xs text-caption text-grey-7">Chat IDs</div>
        <div class="q-mb-xs">
          <q-chip
            v-for="id in form.telegram_chat_ids"
            :key="id"
            removable
            dense
            @remove="removeChatId(id)"
          >
            {{ id }}
          </q-chip>
          <span v-if="form.telegram_chat_ids.length === 0" class="text-grey text-caption">
            No chat IDs configured
          </span>
        </div>

        <q-btn
          outline
          dense
          size="sm"
          label="Get Chat IDs from bot"
          icon="search"
          class="q-mb-md"
          :loading="fetchingChats"
          @click="fetchChats"
        />

        <div v-if="chatsError" class="text-negative text-caption q-mb-sm">{{ chatsError }}</div>

        <q-input
          v-model="alertStatusCodesStr"
          label="Alert Status Codes (comma-separated)"
          outlined
          dense
          class="q-mb-sm"
          hint="e.g. 500,502,503"
        />

        <q-input
          v-model.number="form.alert_cooldown_mins"
          label="Cooldown (minutes)"
          outlined
          dense
          type="number"
          class="q-mb-md"
        />

        <div v-if="saveError" class="text-negative text-caption q-mb-sm">{{ saveError }}</div>

        <div class="row q-gutter-sm">
          <q-btn color="primary" label="Save Settings" :loading="saving" @click="saveSettings" />
          <q-btn outline label="Test Alert" :loading="testing" @click="testAlert" />
        </div>
      </q-card-section>
    </q-card>

    <!-- Chat discovery dialog -->
    <q-dialog v-model="showChatsDialog">
      <q-card style="min-width: 400px">
        <q-card-section>
          <div class="text-h6">Select Chats to Add</div>
        </q-card-section>
        <q-card-section>
          <div v-if="discoveredChats.length === 0" class="text-grey">
            No chats found. Send a message to your bot first.
          </div>
          <q-list v-else dense>
            <q-item
              v-for="chat in discoveredChats"
              :key="chat.chat_id"
              tag="label"
              clickable
            >
              <q-item-section avatar>
                <q-checkbox v-model="selectedChats" :val="chat.chat_id" dense />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ chat.first_name || chat.username || chat.chat_id }}</q-item-label>
                <q-item-label caption>
                  {{ [chat.username ? `@${chat.username}` : null, `ID: ${chat.chat_id}`].filter(Boolean).join(' · ') }}
                </q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" v-close-popup />
          <q-btn
            v-if="discoveredChats.length > 0"
            color="primary"
            label="Add Selected"
            :disable="selectedChats.length === 0"
            @click="addSelectedChats"
            v-close-popup
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useQuasar } from 'quasar';
import { api } from 'boot/axios';

interface Settings {
  telegram_bot_token: string | null;
  telegram_chat_ids: string[];
  alert_status_codes: number[];
  alert_cooldown_mins: number;
}

interface TelegramChat {
  chat_id: string;
  first_name: string | null;
  username: string | null;
}

const $q = useQuasar();

const form = ref<Settings>({
  telegram_bot_token: null,
  telegram_chat_ids: [],
  alert_status_codes: [500, 502, 503],
  alert_cooldown_mins: 5,
});

const alertStatusCodesStr = computed({
  get: () => form.value.alert_status_codes.join(','),
  set: (val: string) => {
    form.value.alert_status_codes = val
      .split(',')
      .map((s) => parseInt(s.trim(), 10))
      .filter((n) => !Number.isNaN(n));
  },
});

const saving = ref(false);
const testing = ref(false);
const fetchingChats = ref(false);
const saveError = ref('');
const chatsError = ref('');
const showChatsDialog = ref(false);
const discoveredChats = ref<TelegramChat[]>([]);
const selectedChats = ref<string[]>([]);

onMounted(async () => {
  try {
    const { data } = await api.get<Settings>('/api/admin/settings');
    form.value = data;
  } catch {
    // use defaults
  }
});

function removeChatId(id: string) {
  form.value.telegram_chat_ids = form.value.telegram_chat_ids.filter((c) => c !== id);
}

async function saveSettings() {
  saving.value = true;
  saveError.value = '';
  try {
    const { data } = await api.put<Settings>('/api/admin/settings', {
      telegram_bot_token: form.value.telegram_bot_token || null,
      telegram_chat_ids: form.value.telegram_chat_ids,
      alert_status_codes: form.value.alert_status_codes,
      alert_cooldown_mins: form.value.alert_cooldown_mins,
    });
    form.value = data;
    $q.notify({ type: 'positive', message: 'Settings saved' });
  } catch (err: unknown) {
    const msg = (err as { response?: { data?: { error?: string } } })?.response?.data?.error ?? 'Failed to save settings';
    saveError.value = msg;
  } finally {
    saving.value = false;
  }
}

async function testAlert() {
  testing.value = true;
  try {
    await api.post('/api/admin/settings/test');
    $q.notify({ type: 'positive', message: 'Test alert sent successfully' });
  } catch (err: unknown) {
    const msg = (err as { response?: { data?: { error?: string } } })?.response?.data?.error ?? 'Test alert failed';
    $q.notify({ type: 'negative', message: msg });
  } finally {
    testing.value = false;
  }
}

async function fetchChats() {
  fetchingChats.value = true;
  chatsError.value = '';
  try {
    const { data } = await api.get<{ chats: TelegramChat[] }>('/api/admin/settings/telegram-chats');
    discoveredChats.value = data.chats;
    selectedChats.value = [];
    showChatsDialog.value = true;
  } catch (err: unknown) {
    const msg = (err as { response?: { data?: { error?: string } } })?.response?.data?.error ?? 'Failed to fetch chats';
    chatsError.value = msg;
  } finally {
    fetchingChats.value = false;
  }
}

function addSelectedChats() {
  const existing = new Set(form.value.telegram_chat_ids);
  for (const id of selectedChats.value) {
    existing.add(id);
  }
  form.value.telegram_chat_ids = Array.from(existing);
}
</script>
