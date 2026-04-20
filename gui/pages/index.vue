<script setup lang="ts">
import type { VaultEntry } from '~/composables/useVault'

const { listEntries } = useVault()

const entries          = ref<VaultEntry[]>([])
const selected         = ref<string | null>(null)
const showForm         = ref(false)
const formInitService  = ref('')
const loadError        = ref<string | null>(null)

async function load() {
  try {
    entries.value = await listEntries()
    if (!selected.value && entries.value.length > 0)
      selected.value = entries.value[0].service
  } catch (e) {
    loadError.value = String(e)
  }
}

function onAccountAdded() {
  showForm.value = false
  load()
}

onMounted(load)
</script>

<template>
  <div class="layout">
    <ServiceList
      :entries="entries"
      :selected="selected"
      @select="selected = $event"
      @add="formInitService = ''; showForm = true"
    />

    <ServiceDetail
      :entries="entries"
      :service="selected"
      @add-account="formInitService = selected ?? ''; showForm = true"
      @refresh="load"
    />

    <Teleport to="body">
      <AddAccountForm
        v-if="showForm"
        :initial-service="formInitService"
        :existing-services="entries.map(e => e.service)"
        @saved="onAccountAdded"
        @close="showForm = false"
      />
    </Teleport>
  </div>
</template>

<style scoped>
.layout {
  display: grid;
  grid-template-columns: 260px 1fr;
  height: 100vh;
  overflow: hidden;
  background: var(--bg);
}
</style>
