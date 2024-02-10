<script setup lang="ts">
const { items } = defineProps<{
  items: Array<{
    id: number;
    is_live: boolean;
    last: unknown;
    name: string;
    realm: string;
    short: string;
  }>;
}>();
</script>

<template>
  <div class="server-status">
    <div v-for="item in items" :key="item.id" class="server-status-item">
      <span class="server-status-icon" :class="{ live: item.is_live }"></span>
      <span>{{ item.realm }}</span>
    </div>
  </div>
</template>

<style scoped>
.server-status {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.server-status-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8px;
  color: white;
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.04);
  border-radius: 4px;
  padding: 8px;
}

.server-status-icon {
  display: block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #ccc;
}

.server-status-icon.live {
  background: #6ddb03;
}
</style>
