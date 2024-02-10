<script setup lang="ts">
import { reactive } from "vue";

const { variant, href } = defineProps<{
  variant?: string;
  href?: string;
}>();

const classObject = reactive({
  [`variant-${variant}`]: !!variant,
});

console.log("variant", variant, href);
</script>

<template>
  <a v-if="href" :class="classObject">
    <slot></slot>
  </a>
  <button v-else type="button" :class="classObject">
    <slot></slot>
  </button>
</template>

<style scoped>
button {
  font-size: 16px;
  position: relative;
  text-decoration: none;
  display: inline-flex;
  font-size: 16px;
  height: 48px;
  appearance: none;
  border: 0;
  color: white;
  background: transparent;
  border: 2px solid transparent;
  border-radius: 8px;
  padding: 0 24px;
  font-size: 16px;
  align-items: center;
  justify-content: center;
  transition: all 200ms ease-out;
  user-select: none;
}

button:hover {
  /* background: #b23300; */
  /* border-color: #b23300; */
  cursor: pointer;
}

.variant-play {
  background: #aa2409;
  height: 56px;
  font-size: 24px;
  font-weight: 500;
  font-family: Object Sans, sans-serif;
  padding: 0 40px;
  border-radius: 4px;
  min-width: 240px;
}

.variant-play:hover {
  border-color: #b23300;
}

button.variant-nav {
  font-size: 18px;
  font-weight: 600;
  background: none;
  border-radius: 8px;
  min-width: 120px;
}

button.variant-nav:hover {
  background: rgba(255, 255, 255, 0.05);
}

a.variant-nav {
  font-size: 18px;
  font-weight: 600;
  padding: 0 16px;
  height: 48px;
  color: white;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  text-decoration: none;
}

a.variant-nav:hover {
  background: rgba(255, 255, 255, 0.05);
}
</style>
