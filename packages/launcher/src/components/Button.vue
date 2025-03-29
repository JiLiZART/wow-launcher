<script setup lang="ts">
import { reactive } from "vue";
import { open } from "@tauri-apps/plugin-shell";

const { variant, text, href, disabled } = defineProps<{
  disabled?: boolean;
  variant?: string;
  text?: string;
  href?: string;
}>();

const emit = defineEmits(["click"]);

const classObject = reactive({
  [`variant-${variant}`]: !!variant,
  [`text-${text}`]: !!text,
  disabled: disabled,
});

const onClick = (e: MouseEvent) => {
  if (href) {
    e.preventDefault();

    open(href);
  } else {
    emit("click", e);
  }
};
</script>

<template>
  <a v-if="href" :class="classObject" href="#" @click="onClick">
    <slot></slot>
  </a>
  <button v-else type="button" :class="classObject" @click="onClick">
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

button.disabled {
  opacity: 0.6;
}

.variant-play {
  background: #aa2409;
  height: 56px;
  font-size: 24px;
  font-weight: 500;
  font-family: Object Sans, sans-serif;
  padding: 0 40px;
  border-radius: 8px;
  min-width: 240px;
}

.variant-play:hover {
  border-color: #b23300;
}

.variant-login {
  height: 48px;
  color: #fff;
  background: #0074e0;
  padding: 0 24px;
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  filter: brightness(100%);
  text-align: center;
  font-size: 16px;
  font-weight: 600;
}

button.variant-install {
  height: 32px;
  color: #fff;
  background: #0074e0;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  text-align: center;
  font-size: 16px;
  font-weight: 600;
}

button.variant-remove {
  height: 32px;
  color: #fff;
  background: #b10000;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  text-align: center;
  font-size: 16px;
  font-weight: 600;
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

a.text-left,
button.text-left {
  justify-content: flex-start;
}
</style>
