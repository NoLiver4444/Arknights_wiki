import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { fetchOperators, type Operator } from '@/api/operators'

export const useOperatorsStore = defineStore('operators', () => {
  const operators = ref<Operator[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const searchQuery = ref('')
  const selectedRarity = ref<number | null>(null)

  async function loadOperators() {
    loading.value = true
    error.value = null
    try {
      operators.value = await fetchOperators()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Неизвестная ошибка'
    } finally {
      loading.value = false
    }
  }

  const filteredOperators = computed(() => {
    return operators.value.filter(op => {
      const matchesSearch = op.name
        .toLowerCase()
        .includes(searchQuery.value.toLowerCase())
      const matchesRarity =
        selectedRarity.value === null || op.rarity === selectedRarity.value
      return matchesSearch && matchesRarity
    })
  })

  return {
    operators,
    loading,
    error,
    searchQuery,
    selectedRarity,
    filteredOperators,
    loadOperators,
  }
})