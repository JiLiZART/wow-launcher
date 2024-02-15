<script setup lang="ts">
import Titlebar from "./Titlebar.vue";
</script>

<template>
  <div id="app" data-tauri-drag-region>
    <Titlebar />
    <div id="container">
      <header>
        <slot name="header"></slot>
      </header>
      <aside>
        <slot name="aside"></slot>
      </aside>
      <main>
        <slot></slot>
      </main>
      <footer>
        <slot name="footer"></slot>
      </footer>
    </div>
  </div>
</template>

<style scoped>
header {
  /* background: rgba(60, 42, 41, 0.8); */
  background: rgba(49, 51, 57, 0.6);
  padding: 12px;
  border-radius: 8px;
}

div#app {
  user-select: none;
  position: relative;
  padding: 24px;
  border-radius: 12px;
  background: url("../assets/background-legion.jpg") no-repeat;
  background-size: cover;
  padding-top: 54px;
  padding-bottom: 12px;
  border: 1px solid #44454c;
  overflow: hidden;
}

div#container {
  display: grid;
  grid-gap: 10px;
  grid-template-columns: 240px 1fr 1fr;
  grid-template-rows: 72px 1fr 140px;
  grid-template-areas:
    "sidebar header header"
    "sidebar content content"
    "footer  content  content";

  width: 100%;
  height: 100%;

  border-radius: 12px;
  min-width: 1280px;
  height: 600px;
}

#container > header {
  grid-area: header;
  display: flex;
  width: 100%;
  justify-content: space-between;
}

#container > main {
  grid-area: content;
}

#container > aside {
  grid-area: sidebar;
  display: flex;
  flex-direction: column;
}

#container > footer {
  color: white;
  grid-area: footer;
  align-items: flex-end;
  display: flex;
}
</style>
