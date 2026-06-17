// Типы данных, которые мы ожидаем от бэкенда
export interface Operator {
  id: string
  name: string
  rarity: number        // 1-6 звёзд
  class: string         // Vanguard, Guard, Defender, Sniper, Caster, Medic, Supporter, Specialist
  avatar: string        // URL картинки
}

// Базовый URL бэкенда (поменяй на свой, когда напишешь Rust-сервер)
const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

/**
 * Получить список всех операторов
 * Ожидаемый ответ от Rust-бэка: GET /api/operators -> Operator[]
 */
export async function fetchOperators(): Promise<Operator[]> {
  const response = await fetch(`${API_BASE}/operators`)

  if (!response.ok) {
    throw new Error(`Ошибка загрузки операторов: ${response.status}`)
  }

  return response.json()
}