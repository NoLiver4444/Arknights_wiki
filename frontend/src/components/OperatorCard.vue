<script setup lang="ts">
import type { Operator } from '@/api/operators'

defineProps<{
  operator: Operator
}>()

function rarityColor(rarity: number): string {
  const colors: Record<number, string> = {
    1: '#9d9d9d',
    2: '#5ebc3a',
    3: '#2cb8dc',
    4: '#b283dc',
    5: '#ffd800',
    6: '#ff6200',
  }
  return colors[rarity] || '#ffffff'
}
</script>

<template>
  <div class="operator-card">
    <!-- Уголки-декор в стиле Arknights -->
    <div class="corner corner-tl"></div>
    <div class="corner corner-tr"></div>
    <div class="corner corner-bl"></div>
    <div class="corner corner-br"></div>

    <div class="card-image">
      <img :src="operator.avatar" :alt="operator.name" loading="lazy" />
      <div class="image-gradient"></div>
    </div>

    <div class="rarity-badge">
      <span
        v-for="star in operator.rarity"
        :key="star"
        class="star"
        :style="{ color: rarityColor(operator.rarity) }"
      >
        ★
      </span>
    </div>

    <div class="class-badge">{{ operator.class }}</div>

    <div class="operator-name">
      <span class="name-text">{{ operator.name }}</span>
    </div>
  </div>
</template>

<style scoped>
.operator-card {
  position: relative;
  aspect-ratio: 3 / 4;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  cursor: pointer;
  overflow: hidden;
  transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1),
              box-shadow 0.3s ease,
              border-color 0.3s ease;
  clip-path: polygon(
    0 12px,
    12px 0,
    calc(100% - 12px) 0,
    100% 12px,
    100% calc(100% - 12px),
    calc(100% - 12px) 100%,
    12px 100%,
    0 calc(100% - 12px)
  );
}

.operator-card:hover {
  transform: scale(1.06) translateY(-4px);
  border-color: var(--accent-gold);
  box-shadow:
    0 10px 30px rgba(245, 197, 24, 0.25),
    0 0 0 1px var(--accent-gold);
  z-index: 10;
}

.corner {
  position: absolute;
  width: 14px;
  height: 14px;
  border: 2px solid var(--accent-gold);
  z-index: 3;
  opacity: 0.6;
  transition: opacity 0.3s ease;
}
.operator-card:hover .corner {
  opacity: 1;
}
.corner-tl { top: 4px; left: 4px; border-right: none; border-bottom: none; }
.corner-tr { top: 4px; right: 4px; border-left: none; border-bottom: none; }
.corner-bl { bottom: 4px; left: 4px; border-right: none; border-top: none; }
.corner-br { bottom: 4px; right: 4px; border-left: none; border-top: none; }

.card-image {
  position: absolute;
  inset: 0;
  overflow: hidden;
}
.card-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}
.operator-card:hover .card-image img {
  transform: scale(1.1);
}

.image-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    to top,
    rgba(10, 10, 12, 0.95) 0%,
    rgba(10, 10, 12, 0.6) 35%,
    rgba(10, 10, 12, 0) 60%
  );
}

.rarity-badge {
  position: absolute;
  top: 10px;
  left: 10px;
  display: flex;
  gap: 2px;
  z-index: 2;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.8));
}
.star {
  font-size: 14px;
  line-height: 1;
}

.class-badge {
  position: absolute;
  top: 10px;
  right: 10px;
  background-color: rgba(10, 10, 12, 0.75);
  border: 1px solid var(--border-color);
  padding: 3px 8px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 1px;
  text-transform: uppercase;
  color: var(--text-secondary);
  z-index: 2;
  font-family: var(--font-display);
}

.operator-name {
  position: absolute;
  bottom: 10px;
  right: 12px;
  z-index: 2;
  text-align: right;
  max-width: 80%;
}
.name-text {
  font-family: var(--font-display);
  font-size: 18px;
  font-weight: 700;
  letter-spacing: 1px;
  text-transform: uppercase;
  color: var(--text-primary);
  text-shadow: 0 2px 6px rgba(0, 0, 0, 0.9);
  line-height: 1.1;
  word-break: break-word;
}

.operator-card:hover .name-text {
  color: var(--accent-gold);
}
</style>