<script setup lang="ts"></script>

<template>
  <div class="server-status">
    <div class="server-status-item">
      <span class="server-status-icon"></span>
      <span>Scourge x2</span>
    </div>
    <div class="server-status-item">
      <span class="server-status-icon"></span>
      <span>Algalon x4</span>
    </div>
    <div class="server-status-item">
      <span class="server-status-icon"></span>
      <span>Soulseeker x1</span>
    </div>
    <div class="server-status-item">
      <span class="server-status-icon"></span>
      <span>Sirus x5</span>
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
  background: #6ddb03;
}
</style>
