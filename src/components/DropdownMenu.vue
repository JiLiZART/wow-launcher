<script setup lang="ts">
import { ref, useSlots } from "vue";

const slots = useSlots();

const { openOn } = defineProps<{
  openOn: "hover" | "click";
}>();

const isOpen = ref(false);
const offset = ref("-136px");

const onOpen = () => {
  if (openOn === "click") {
    isOpen.value = !isOpen.value;
  }
};

const onEnter = () => {
  if (openOn === "hover") {
    isOpen.value = true;
  }
};

const onLeave = () => {
  setTimeout(() => {
    if (openOn === "hover") {
      isOpen.value = false;
    }
  }, 1000);
};
const withSecondary = slots?.secondary?.();
</script>

<template>
  <div class="dropdown-button" @mouseenter="onEnter" @mouseleave="onLeave">
    <slot :open="onOpen" :is-open="isOpen"></slot>
    <div
      class="dropdown-menu"
      :class="{ 'is-open': isOpen }"
      :style="{ left: offset }"
    >
      <div class="dropdown-arrow" />
      <div class="dropdown-menu-primary" :class="{ single: !withSecondary }">
        <slot name="primary">No contents</slot>
      </div>
      <div v-if="withSecondary" class="dropdown-menu-secondary">
        <slot name="secondary"> </slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dropdown-button {
  position: relative;
  perspective: 1000px;
  display: inline-block;
  z-index: 4;
}

.dropdown-menu {
  clip-path: polygon(0px 0px, 0px 0px, 0px 0px, 0px 0px);
  max-height: 0px;
  overflow: hidden;
  padding: 0px;
  transition: transform 0.25s ease 0s, opacity 0.25s ease 0s,
    max-height 0.25s ease 0s;

  display: none;
  height: auto;
  max-height: none;
  min-width: 320px;
  opacity: 1;
  padding-top: 16px;
  position: absolute;
  top: 100%;
  transform: rotateX(-15deg);
  transform-origin: 50% -50px;
  width: auto;
}

.dropdown-menu::before {
  /* background: #2b1d1c; */
  background: rgba(49, 51, 57, 1);
  border-radius: 8px;
  content: "";
  height: calc(100% - 16px);
  left: 0px;
  position: absolute;
  top: 16px;
  width: 100%;
  z-index: -1;
}

.dropdown-button.hover-open:hover .dropdown-menu,
.dropdown-menu.is-open {
  animation: fadeIn 0.25s;
  clip-path: polygon(100% 0, 100% 100%, 0 100%, 0 0);
  display: block;
  opacity: 1;
  transform: rotateX(0deg);
}

.dropdown-arrow {
  background: #2b1d1c;
  background: rgba(49, 51, 57, 1);
  border-color: #000 #0000 #0000 #000;
  border-radius: 4px;
  border-style: solid;
  border-width: 0px;
  height: 16px;
  left: 50%;
  position: absolute;
  top: 14px;
  transform: rotate(45deg) translate(-50%);
  transform-origin: center center;
  transition: opacity 100ms ease;
  width: 16px;
  z-index: 1;
}

.dropdown-menu-primary {
  border-radius: 8px 8px 0 0;

  display: flex;
  justify-content: space-between;
  white-space: nowrap;

  display: flex;
  flex-direction: column;
  color: white;
  position: relative;
  padding: 12px;
}

.dropdown-menu-primary.single {
  border-radius: 8px;
}

.dropdown-menu-secondary {
  background: #0000004d;
  display: flex;
  flex-direction: column;
  color: white;
  position: relative;
  padding: 12px;
  border-radius: 0 0 8px 8px;
}

@keyframes fadeIn {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
</style>
