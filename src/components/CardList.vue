<script setup lang="ts">
import { getCoverUrl, NewsItem } from "../api";

const { items } = defineProps<{
  items: NewsItem[];
}>();

const emit = defineEmits<{
  show: [NewsItem];
}>();
</script>

<template>
  <div class="card-list">
    <div
      v-for="item in items"
      class="card"
      :key="item.id"
      @click="emit('show', item)"
    >
      <div class="card-image">
        <img :src="getCoverUrl(item.cover_url)" :alt="item.title" />
      </div>
      <div class="card-meta">
        <div class="card-category">{{ item.category.name }}</div>
        <div class="card-title">
          {{ item.title }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card-list {
  display: flex;
  flex-direction: row;
  gap: 12px;
  flex-wrap: wrap;
}

.card {
  border-radius: 8px;
  overflow: hidden;
  width: 340px;
  height: 340px;
  display: flex;
  flex-direction: column;
  justify-content: stretch;
}

.card:hover {
  cursor: pointer;
}

.card-image {
  border-radius: 8px 8px 0 0;
  overflow: hidden;
  flex-grow: 1;
}

.card-image img {
  object-fit: cover;
  width: 100%;
  height: 100%;

  transition: opacity 0.35s, transform 0.35s;
  transform: scale(1.15);
}

.card:hover .card-image img {
  transform: scale(1);
}

.card-category {
  font-size: 14px;
  text-transform: uppercase;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-title {
  font-size: 18px;
  font-weight: 700;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-meta {
  /* background: #22252b; */
  background: rgba(49, 51, 57, 0.6);
  padding: 20px;
  color: white;
  height: 120px;
  transition: background 200ms ease-in;
}

.card:hover .card-meta {
  background: rgba(49, 51, 57, 0.7);
}
</style>
