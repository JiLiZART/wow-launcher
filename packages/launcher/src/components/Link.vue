<script setup lang="ts">
import { open } from "@tauri-apps/plugin-shell";

const { href } = defineProps<{
  href: string;
}>();

const onClick = (e: MouseEvent) => {
  if (href) {
    e.preventDefault();

    open(href);
  }
};
</script>

<template>
  <a @click="onClick">
    <slot></slot>
  </a>
</template>

<style scoped>
a:hover {
  cursor: pointer;
}
</style>
