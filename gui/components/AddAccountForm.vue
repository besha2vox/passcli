<script setup lang="ts">
const props = defineProps<{
  initialService: string
  existingServices: string[]
}>()

const emit = defineEmits<{
  saved: []
  close: []
}>()

const { addAccount, generatePassword } = useVault()

/* ── Form fields ── */
const service   = ref(props.initialService)
const username  = ref('')
const password  = ref('')

/* ── Mode: generate | manual ── */
type Mode = 'generate' | 'manual'
const mode = ref<Mode>('generate')

/* ── Generator options ── */
const genCase     = ref<'mixed' | 'lower' | 'upper'>('mixed')
const genLength   = ref(16)
const genSpecials = ref(true)
const genPreview  = ref('')
const generating  = ref(false)
const showPass    = ref(false)

/* ── State ── */
const saving      = ref(false)
const error       = ref<string | null>(null)

const isNewService = computed(() => !props.existingServices.includes(service.value))

const canSave = computed(() =>
  service.value.trim() &&
  username.value.trim() &&
  password.value.length > 0
)

async function generate() {
  generating.value = true
  try {
    genPreview.value = await generatePassword(genLength.value, genCase.value, genSpecials.value)
    password.value   = genPreview.value
  } finally {
    generating.value = false
  }
}

function useGenerated() {
  password.value  = genPreview.value
}

async function save() {
  if (!canSave.value) return
  error.value = null
  saving.value = true
  try {
    await addAccount(service.value.trim(), username.value.trim(), password.value)
    emit('saved')
  } catch (e) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}

function onBackdrop(e: MouseEvent) {
  if (e.target === e.currentTarget) emit('close')
}
</script>

<template>
  <div class="backdrop" @click="onBackdrop">
    <div class="modal">
      <!-- Header -->
      <div class="modal-header">
        <h3>Додати акаунт</h3>
        <button class="close-btn" @click="emit('close')">✕</button>
      </div>

      <!-- Body -->
      <div class="modal-body">
        <!-- Service -->
        <div class="field">
          <label>Сервіс</label>
          <input
            v-model="service"
            placeholder="google, github..."
            :readonly="!isNewService && !!initialService"
          />
          <span v-if="isNewService && service" class="hint">Буде створено новий сервіс</span>
        </div>

        <!-- Username -->
        <div class="field">
          <label>Юзернейм / Email</label>
          <input v-model="username" placeholder="user@example.com" />
        </div>

        <!-- Password mode tabs -->
        <div class="field">
          <div class="tabs">
            <button
              :class="{ active: mode === 'generate' }"
              @click="mode = 'generate'"
            >Генерувати</button>
            <button
              :class="{ active: mode === 'manual' }"
              @click="mode = 'manual'"
            >Ввести вручну</button>
          </div>

          <!-- Generate mode -->
          <div v-if="mode === 'generate'" class="gen-panel">
            <div class="gen-options">
              <div class="option-group">
                <span class="option-label">Регістр</span>
                <div class="radio-row">
                  <label>
                    <input v-model="genCase" type="radio" value="lower" />
                    нижній
                  </label>
                  <label>
                    <input v-model="genCase" type="radio" value="upper" />
                    верхній
                  </label>
                  <label>
                    <input v-model="genCase" type="radio" value="mixed" />
                    змішаний
                  </label>
                </div>
              </div>

              <div class="option-row">
                <label class="checkbox-label">
                  <input v-model="genSpecials" type="checkbox" />
                  Спецсимволи (!@#$...)
                </label>
              </div>

              <div class="option-row length-row">
                <span class="option-label">Довжина</span>
                <input
                  v-model.number="genLength"
                  type="number"
                  min="4"
                  max="128"
                  class="length-input"
                />
              </div>
            </div>

            <button
              class="btn-generate"
              :disabled="generating"
              @click="generate"
            >
              {{ generating ? '...' : '⚡ Генерувати' }}
            </button>

            <div v-if="genPreview" class="preview">
              <span class="preview-pass">{{ genPreview }}</span>
            </div>
          </div>

          <!-- Manual mode -->
          <div v-else class="manual-panel">
            <div class="pass-input-wrap">
              <input
                v-model="password"
                :type="showPass ? 'text' : 'password'"
                placeholder="Введіть пароль"
                autocomplete="new-password"
              />
              <button class="eye-btn" @click="showPass = !showPass">
                {{ showPass ? '🙈' : '👁' }}
              </button>
            </div>
          </div>
        </div>

        <!-- Error -->
        <p v-if="error" class="error-msg">{{ error }}</p>
      </div>

      <!-- Footer -->
      <div class="modal-footer">
        <button class="btn-cancel" @click="emit('close')">Скасувати</button>
        <button
          class="btn-save"
          :disabled="!canSave || saving"
          @click="save"
        >
          {{ saving ? 'Зберігаємо...' : 'Зберегти' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  animation: fade-in 0.15s ease;
}

@keyframes fade-in { from { opacity: 0; } to { opacity: 1; } }

.modal {
  background: var(--panel);
  border: 1px solid var(--border-strong);
  border-radius: var(--radius);
  width: 420px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 60px rgba(0,0,0,0.6);
  animation: slide-up 0.18s ease;
}

@keyframes slide-up {
  from { transform: translateY(12px); opacity: 0; }
  to   { transform: translateY(0);    opacity: 1; }
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 14px;
  border-bottom: 1px solid var(--border);
}

.modal-header h3 { font-size: 15px; font-weight: 700; }

.close-btn {
  width: 26px;
  height: 26px;
  border-radius: 6px;
  color: var(--text-muted);
  font-size: 12px;
}
.close-btn:hover { background: var(--surface-hover); color: var(--text); }

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* Fields */
.field { display: flex; flex-direction: column; gap: 7px; }

.field label,
.option-label {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-secondary);
}

.hint { font-size: 11px; color: var(--accent); }

/* Tabs */
.tabs {
  display: flex;
  gap: 2px;
  background: var(--surface);
  border-radius: var(--radius-sm);
  padding: 3px;
  margin-bottom: 12px;
}

.tabs button {
  flex: 1;
  padding: 6px;
  border-radius: calc(var(--radius-sm) - 2px);
  font-size: 13px;
  color: var(--text-muted);
}

.tabs button.active {
  background: var(--surface-hover);
  color: var(--text);
}

.tabs button:hover:not(.active) { color: var(--text-secondary); }

/* Generate panel */
.gen-panel { display: flex; flex-direction: column; gap: 14px; }

.gen-options {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 14px;
  background: var(--surface);
  border-radius: var(--radius-sm);
}

.option-group { display: flex; flex-direction: column; gap: 7px; }

.radio-row {
  display: flex;
  gap: 16px;
}

.radio-row label,
.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
}

.radio-row input[type="radio"],
.checkbox-label input[type="checkbox"] {
  width: auto;
  accent-color: var(--accent);
  cursor: pointer;
}

.option-row { display: flex; align-items: center; gap: 10px; }

.length-row { justify-content: space-between; }

.length-input {
  width: 68px;
  text-align: center;
  padding: 6px 8px;
}

.btn-generate {
  padding: 9px;
  border-radius: var(--radius-sm);
  background: var(--accent);
  color: #fff;
  font-size: 13.5px;
  font-weight: 600;
  transition: background var(--transition);
}
.btn-generate:hover:not(:disabled) { background: var(--accent-hover); }
.btn-generate:disabled { opacity: 0.5; cursor: not-allowed; }

.preview {
  padding: 10px 14px;
  background: var(--surface);
  border: 1px solid var(--border-strong);
  border-radius: var(--radius-sm);
}

.preview-pass {
  font-family: 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  color: var(--green);
  word-break: break-all;
}

/* Manual panel */
.manual-panel { position: relative; }

.pass-input-wrap { position: relative; }

.pass-input-wrap input { padding-right: 40px; }

.eye-btn {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 15px;
  color: var(--text-muted);
  padding: 0;
}
.eye-btn:hover { color: var(--text); }

/* Error */
.error-msg {
  font-size: 12.5px;
  color: var(--red);
  padding: 8px 12px;
  background: rgba(224,92,92,0.1);
  border-radius: var(--radius-sm);
}

/* Footer */
.modal-footer {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 14px 20px 18px;
  border-top: 1px solid var(--border);
}

.btn-cancel {
  padding: 8px 18px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 13.5px;
}
.btn-cancel:hover { background: var(--surface-hover); }

.btn-save {
  padding: 8px 22px;
  border-radius: var(--radius-sm);
  background: var(--accent);
  color: #fff;
  font-size: 13.5px;
  font-weight: 600;
}
.btn-save:hover:not(:disabled) { background: var(--accent-hover); }
.btn-save:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
