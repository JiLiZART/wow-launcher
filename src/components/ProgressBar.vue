<script setup lang="ts">
import PlayIcon from "./PlayIcon.vue";
import Button from "./Button.vue";
</script>

<template>
  <div class="progress">
    <div class="progress-content">
      <div class="progress-bar">
        <div class="progress-bar-line"></div>
      </div>
      <Button class="progress-action"><PlayIcon /></Button>
    </div>
    <div class="progress-meta">
      <span>99% (25 GB)</span> <span>2.5 MB/s</span>
    </div>
  </div>
</template>

<style scoped>
.progress {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0px;
}

.progress-content {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
}

.progress-meta {
  font-size: 14px;
  display: flex;
  width: 100%;
  justify-content: space-between;
}

.progress-action {
  padding: 0;
  height: auto;
  width: auto;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #3f4147;
  border-radius: 4px;
  transition: 0.4s linear;
  transition-property: width, background-color;
}

.progress-bar-line {
  height: 8px;
  width: 50%;
  border-radius: 4px;
  background: #6d6e72;
  transition: 0.4s linear;
  transition-property: width, background-color;
}
</style>
