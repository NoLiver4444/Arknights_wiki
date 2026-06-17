<script setup lang="ts">
import { onMounted } from 'vue'
import { useOperatorsStore } from '@/stores/operators'
import OperatorCard from '@/components/OperatorCard.vue'

const store = useOperatorsStore()

onMounted(() => {
  if (store.operators.length === 0) {
    store.loadOperators()
  }
})

const rarities = [6, 5, 4, 3, 2, 1]
</script>

<template>
  <div class="operators-page">
    <header class="page-header">
      <div class="container">
        <div class="header-content">
          <div class="logo-block">
            <div class="logo-accent"></div>
            <h1 class="logo-text">RHODES ISLAND</h1>
            <span class="logo-sub">OPERATORS DATABASE</span>
          </div>
        </div>
      </div>
    </header>

    <section class="filters-section">
      <div class="container">
        <div class="filters-bar">
          <div class="search-box">
            <span class="search-icon">⌕</span>
            <input
              v-model="store.searchQuery"
              type="text"
              placeholder="Поиск оператора..."
              class="search-input"
            />
          </div>

          <div class="rarity-filter">
            <button
              class="rarity-btn"
              :class="{ active: store.selectedRarity === null }"
              @click="store.selectedRarity = null"
            >
              ВСЕ
            </button>
            <button
              v-for="r in rarities"
              :key="r"
              class="rarity-btn"
              :class="{ active: store.selectedRarity === r }"
              @click="store.selectedRarity = r"
            >
              {{ '★'.repeat(r) }}
            </button>
          </div>
        </div>
      </div>
    </section>

    <main class="content">
      <div class="container">
        <!-- Загрузка -->
        <div v-if="store.loading" class="state-block">
          <div class="loader"></div>
          <p class="state-text">ЗАГРУЗКА ДАННЫХ ИЗ RHODES ISLAND...</p>
        </div>

        <div v-else-if="store.error" class="state-block error">
          <div class="error-icon">⚠</div>
          <p class="state-text">ОШИБКА СВЯЗИ</p>
          <p class="state-sub">{{ store.error }}</p>
          <button class="retry-btn" @click="store.loadOperators()">
            ПОВТОРИТЬ ЗАПРОС
          </button>
        </div>

        <div v-else-if="store.filteredOperators.length === 0" class="state-block">
          <p class="state-text">ОПЕРАТОРЫ НЕ НАЙДЕНЫ</p>
          <p class="state-sub">Попробуйте изменить параметры поиска</p>
        </div>

        <div v-else class="operators-grid">
          <OperatorCard
            v-for="op in store.filteredOperators"
            :key="op.id"
            :operator="op"
          />
        </div>

        <div v-if="!store.loading && !store.error" class="counter">
          <span class="counter-label">ВСЕГО ОПЕРАТОРОВ:</span>
          <span class="counter-value">{{ store.filteredOperators.length }}</span>
          <span v-if="store.searchQuery || store.selectedRarity" class="counter-total">
            / {{ store.operators.length }}
          </span>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.page-header {
  background: linear-gradient(180deg, #141418 0%, #0a0a0c 100%);
  border-bottom: 1px solid var(--border-color);
  padding: 32px 0;
  position: relative;
}

.page-header::before {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, var(--accent-gold), transparent);
}

.header-content {
  display: flex;
  align-items: center;
}

.logo-block {
  display: flex;
  align-items: baseline;
  gap: 16px;
  position: relative;
}

.logo-accent {
  width: 6px;
  height: 40px;
  background: var(--accent-gold);
  transform: skewX(-20deg);
}

.logo-text {
  font-family: var(--font-display);
  font-size: 32px;
  font-weight: 900;
  letter-spacing: 4px;
  color: var(--text-primary);
}

.logo-sub {
  font-family: var(--font-display);
  font-size: 13px;
  letter-spacing: 3px;
  color: var(--accent-gold);
  padding-left: 16px;
  border-left: 1px solid var(--border-color);
}

.filters-section {
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  padding: 20px 0;
  position: sticky;
  top: 0;
  z-index: 50;
  backdrop-filter: blur(10px);
}

.filters-bar {
  display: flex;
  gap: 24px;
  align-items: center;
  flex-wrap: wrap;
}

.search-box {
  flex: 1;
  min-width: 250px;
  max-width: 400px;
  position: relative;
}

.search-icon {
  position: absolute;
  left: 14px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-secondary);
  font-size: 20px;
  pointer-events: none;
}

.search-input {
  width: 100%;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  padding: 12px 16px 12px 44px;
  font-family: var(--font-body);
  font-size: 15px;
  font-weight: 500;
  letter-spacing: 0.5px;
  outline: none;
  transition: border-color 0.2s ease;
  clip-path: polygon(0 0, calc(100% - 10px) 0, 100% 10px, 100% 100%, 10px 100%, 0 calc(100% - 10px));
}

.search-input:focus {
  border-color: var(--accent-gold);
}

.search-input::placeholder {
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
  font-size: 13px;
}

.rarity-filter {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.rarity-btn {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 8px 12px;
  font-family: var(--font-display);
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.rarity-btn:hover {
  border-color: var(--accent-gold);
  color: var(--accent-gold);
}

.rarity-btn.active {
  background: var(--accent-gold);
  color: var(--bg-primary);
  border-color: var(--accent-gold);
}

/* Контент */
.content {
  padding: 40px 0 60px;
  flex: 1;
}

.operators-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 20px;
}

@media (min-width: 768px) {
  .operators-grid {
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 24px;
  }
}

.state-block {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  text-align: center;
}

.loader {
  width: 50px;
  height: 50px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-gold);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 24px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.state-text {
  font-family: var(--font-display);
  font-size: 20px;
  font-weight: 700;
  letter-spacing: 3px;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.state-sub {
  color: var(--text-secondary);
  font-size: 14px;
  letter-spacing: 1px;
}

.error-icon {
  font-size: 48px;
  color: var(--accent-red);
  margin-bottom: 16px;
}

.retry-btn {
  margin-top: 24px;
  background: transparent;
  border: 1px solid var(--accent-gold);
  color: var(--accent-gold);
  padding: 10px 24px;
  font-family: var(--font-display);
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 2px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.retry-btn:hover {
  background: var(--accent-gold);
  color: var(--bg-primary);
}

.counter {
  margin-top: 40px;
  padding-top: 24px;
  border-top: 1px solid var(--border-color);
  text-align: center;
  font-family: var(--font-display);
  letter-spacing: 2px;
}

.counter-label {
  color: var(--text-secondary);
  font-size: 13px;
  margin-right: 8px;
}

.counter-value {
  color: var(--accent-gold);
  font-size: 20px;
  font-weight: 700;
}

.counter-total {
  color: var(--text-secondary);
  font-size: 14px;
  margin-left: 4px;
}
</style>