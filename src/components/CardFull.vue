<script setup lang="ts">
import { getCoverUrl, NewsItem } from "../api";
import Button from "./Button.vue";
const { item } = defineProps<{
  item: NewsItem;
}>();

const emit = defineEmits(["back"]);
</script>

<template>
  <div class="card-full-container">
    <div class="card-full">
      <div class="card-full-image">
        <img :src="getCoverUrl(item.cover_url)" alt="" />
      </div>
      <div class="card-full-category">{{ item.category.name }}</div>
      <div class="card-full-title">{{ item.title }}</div>
      <div class="card-full-text" v-html="item.description"></div>
      <div class="card-full-actions">
        <a href="#">Discuss</a>
        <Button @click="emit('back')">Back</Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card-full-container {
  max-height: 500px;
  overflow-y: scroll;
  overflow-x: hidden;
  padding: 8px;
  background: rgba(49, 51, 57, 0.8);
  border-radius: 8px;
}

.card-full {
  display: flex;
  width: 1000px;
  flex-direction: column;
}

.card-full-title {
  color: white;
  font-size: 52px;
  font-weight: 700;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-full-category {
  margin-top: 12px;
  font-size: 18px;
  color: white;
  text-transform: uppercase;
  font-weight: 400;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}
.card-full-text {
  line-height: 1.5;
  color: white;
  font-size: 18px;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-full-image {
  width: 100%;
  border: 1px solid rgba(0, 0, 0, 0.2);
  height: 320px;
  border-radius: 8px;
  overflow: hidden;
}
.card-full-image img {
  object-fit: cover;
  width: 100%;
  height: 100%;
}
</style>
