<script setup lang="ts">
import type { VaultEntry } from '~/composables/useVault'

const props = defineProps<{
  entries: VaultEntry[]
  selected: string | null
}>()

const emit = defineEmits<{
  select: [service: string]
  add: []
}>()

const { isFavorite, toggleFavorite } = useFavorites()
const search = ref('')

const sorted = computed(() => {
  const q = search.value.toLowerCase()
  return [...props.entries]
    .filter(e => e.service.toLowerCase().includes(q))
    .sort((a, b) => {
      const af = isFavorite(a.service), bf = isFavorite(b.service)
      if (af !== bf) return af ? -1 : 1
      return a.service.localeCompare(b.service)
    })
})

function onStar(e: MouseEvent, service: string) {
  e.stopPropagation()
  toggleFavorite(service)
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="brand">PassHolder</span>
    </div>

    <div class="search-row">
      <div class="search-wrap">
        <span class="search-icon">⌕</span>
        <input
          v-model="search"
          placeholder="Пошук сервісів..."
          class="search-input"
        />
      </div>
      <button class="btn-add" title="Новий сервіс" @click="emit('add')">+</button>
    </div>

    <ul class="service-list">
      <li
        v-for="entry in sorted"
        :key="entry.service"
        class="service-item"
        :class="{ active: entry.service === selected }"
        @click="emit('select', entry.service)"
      >
        <button
          class="star-btn"
          :class="{ starred: isFavorite(entry.service) }"
          @click="onStar($event, entry.service)"
        >{{ isFavorite(entry.service) ? '★' : '☆' }}</button>

        <span class="service-name">{{ entry.service }}</span>

        <span v-if="entry.accounts.length > 1" class="badge">
          {{ entry.accounts.length }}
        </span>
      </li>
    </ul>

    <div v-if="sorted.length === 0" class="empty-list">
      {{ search ? 'Нічого не знайдено' : 'Немає записів' }}
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  background: var(--panel);
  border-right: 1px solid var(--border);
  overflow: hidden;
}

.sidebar-header {
  padding: 18px 16px 10px;
  border-bottom: 1px solid var(--border);
}

.brand {
  font-size: 15px;
  font-weight: 700;
  color: var(--accent);
  letter-spacing: 0.03em;
}

.search-row {
  display: flex;
  gap: 8px;
  padding: 12px 12px 8px;
}

.search-wrap {
  position: relative;
  flex: 1;
}

.search-icon {
  position: absolute;
  left: 9px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  font-size: 16px;
  pointer-events: none;
}

.search-input {
  padding-left: 30px;
  font-size: 13px;
}

.btn-add {
  width: 34px;
  height: 34px;
  border-radius: var(--radius-sm);
  background: var(--accent);
  color: #fff;
  font-size: 20px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.btn-add:hover { background: var(--accent-hover); }

.service-list {
  flex: 1;
  overflow-y: auto;
  list-style: none;
  padding: 4px 8px;
}

.service-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 9px 10px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition);
}

.service-item:hover { background: var(--surface-hover); }

.service-item.active {
  background: var(--accent-dim);
  color: #fff;
}

.star-btn {
  font-size: 15px;
  color: var(--text-muted);
  line-height: 1;
  padding: 0;
  width: 18px;
  flex-shrink: 0;
  transition: color var(--transition), transform var(--transition);
}

.star-btn:hover { color: #f5c542; transform: scale(1.2); }
.star-btn.starred { color: #f5c542; }

.service-name {
  flex: 1;
  font-size: 13.5px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.badge {
  font-size: 11px;
  background: var(--surface);
  color: var(--text-secondary);
  padding: 1px 6px;
  border-radius: 99px;
  flex-shrink: 0;
}

.service-item.active .badge {
  background: rgba(255,255,255,0.12);
  color: #fff;
}

.empty-list {
  padding: 24px 16px;
  text-align: center;
  color: var(--text-muted);
  font-size: 13px;
}
</style>
