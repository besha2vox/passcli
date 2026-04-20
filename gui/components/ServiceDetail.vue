<script setup lang="ts">
import type { VaultEntry } from '~/composables/useVault'

const props = defineProps<{
  entries: VaultEntry[]
  service: string | null
}>()

const emit = defineEmits<{
  addAccount: []
  refresh: []
}>()

const { copyPassword, removeAccount } = useVault()

const entry = computed<VaultEntry | null>(() =>
  props.entries.find(e => e.service === props.service) ?? null
)

const revealed  = ref<Set<string>>(new Set())
const copied    = ref<string | null>(null)

function toggleReveal(key: string) {
  const s = new Set(revealed.value)
  s.has(key) ? s.delete(key) : s.add(key)
  revealed.value = s
}

async function onCopy(username: string) {
  if (!props.service) return
  try {
    await copyPassword(props.service, username)
    const key = `${props.service}:${username}`
    copied.value = key
    setTimeout(() => { copied.value = null }, 1800)
  } catch (e) {
    console.error(e)
  }
}

async function onRemove(username: string) {
  if (!props.service) return
  await removeAccount(props.service, username)
  emit('refresh')
}
</script>

<template>
  <main class="detail">
    <!-- Empty state -->
    <div v-if="!entry" class="empty">
      <div class="empty-icon">🔐</div>
      <p>Оберіть сервіс або<br>додайте перший запис</p>
    </div>

    <!-- Service content -->
    <template v-else>
      <div class="detail-header">
        <h2 class="service-title">{{ entry.service }}</h2>
        <span class="account-count">
          {{ entry.accounts.length }}
          {{ entry.accounts.length === 1 ? 'акаунт' : 'акаунти' }}
        </span>
      </div>

      <div class="accounts-scroll">
        <div
          v-for="(account, i) in entry.accounts"
          :key="account.username"
          class="account-card"
        >
          <div class="account-row">
            <span class="field-label">User</span>
            <span class="field-value username">{{ account.username }}</span>
          </div>

          <div class="account-row">
            <span class="field-label">Pass</span>
            <span class="field-value password-wrap">
              <span class="password-value">
                {{ revealed.has(account.username) ? account.password : '•'.repeat(12) }}
              </span>
              <div class="pass-actions">
                <button
                  class="icon-btn"
                  :title="revealed.has(account.username) ? 'Сховати' : 'Показати'"
                  @click="toggleReveal(account.username)"
                >
                  {{ revealed.has(account.username) ? '🙈' : '👁' }}
                </button>
                <button
                  class="icon-btn copy-btn"
                  :class="{ copied: copied === `${entry.service}:${account.username}` }"
                  title="Копіювати пароль"
                  @click="onCopy(account.username)"
                >
                  <i
                    :class="copied === `${entry.service}:${account.username}`
                      ? 'fa-solid fa-check'
                      : 'fa-regular fa-copy'"
                  />
                </button>
              </div>
            </span>
          </div>

          <button class="remove-btn" title="Видалити акаунт" @click="onRemove(account.username)">
            ✕
          </button>

          <div v-if="i < entry.accounts.length - 1" class="account-divider" />
        </div>
      </div>

      <div class="detail-footer">
        <button class="btn-add-account" @click="emit('addAccount')">
          + Додати акаунт
        </button>
      </div>
    </template>
  </main>
</template>

<style scoped>
.detail {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--bg);
}

/* Empty */
.empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-muted);
  text-align: center;
  line-height: 1.7;
}

.empty-icon { font-size: 40px; opacity: 0.4; }

/* Header */
.detail-header {
  display: flex;
  align-items: baseline;
  gap: 10px;
  padding: 20px 24px 16px;
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}

.service-title {
  font-size: 17px;
  font-weight: 700;
  color: var(--text);
}

.account-count {
  font-size: 12px;
  color: var(--text-muted);
}

/* Accounts */
.accounts-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
  display: flex;
  flex-direction: column;
  gap: 0;
}

.account-card {
  position: relative;
  padding: 14px 0;
}

.account-row {
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: 28px;
}

.field-label {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  width: 34px;
  flex-shrink: 0;
}

.field-value {
  font-size: 13.5px;
  color: var(--text);
}

.username { font-weight: 500; }

.password-wrap {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
}

.password-value {
  font-family: 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  letter-spacing: 0.04em;
  color: var(--text-secondary);
  flex: 1;
}

.pass-actions { display: flex; gap: 4px; }

.icon-btn {
  width: 28px;
  height: 28px;
  border-radius: var(--radius-sm);
  font-size: 13px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  background: transparent;
}

.icon-btn:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.copy-btn.copied {
  color: var(--green);
  background: var(--green-dim);
}

.remove-btn {
  position: absolute;
  top: 14px;
  right: 0;
  width: 24px;
  height: 24px;
  border-radius: var(--radius-sm);
  font-size: 11px;
  color: var(--text-muted);
  opacity: 0;
  transition: opacity var(--transition), color var(--transition), background var(--transition);
}

.account-card:hover .remove-btn { opacity: 1; }
.remove-btn:hover { color: var(--red); background: rgba(224,92,92,0.12); }

.account-divider {
  height: 1px;
  background: var(--divider);
  margin-top: 14px;
}

/* Footer */
.detail-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}

.btn-add-account {
  width: 100%;
  padding: 9px 16px;
  border-radius: var(--radius-sm);
  border: 1px dashed var(--border-strong);
  color: var(--text-secondary);
  font-size: 13.5px;
  text-align: center;
  transition: border-color var(--transition), color var(--transition), background var(--transition);
}

.btn-add-account:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-dim);
}
</style>
