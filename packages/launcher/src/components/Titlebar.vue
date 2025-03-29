<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const appWindow = getCurrentWebviewWindow();

const close = () => {
  appWindow.close();
  alert("close");
};

const minimize = () => {
  appWindow.minimize();
  alert("minimize");
};
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-controls">
      <div class="titlebar-minimize" @click="minimize">
        <svg x="0px" y="0px" view-box="0 0 10 1">
          <rect fill="#fff" width="10" height="1"></rect>
        </svg>
      </div>
      <div class="titlebar-close" @click="close">
        <svg x="0px" y="0px" view-box="0 0 10 10">
          <polygon
            fill="#fff"
            points="10,1 9,0 5,4 1,0 0,1 4,5 0,9 1,10 5,6 9,10 10,9 6,5"
          ></polygon>
        </svg>
      </div>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  --titlebar-height: 30px;
  --menu-height: 30px;
  --titlebar-font-size: 14px;
  --menu-font-size: 14px;
  --menu-options-max-height: 95vh;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 9999;
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  align-items: flex-end;
  background-color: rgba(255, 255, 255, 0.07);
  height: calc(var(--titlebar-height));
  border-bottom: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 8px 8px 0 0;
}

.titlebar-controls {
  display: flex;
  gap: 0px;
  flex-direction: row;
}

.titlebar-minimize,
.titlebar-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: calc(var(--titlebar-height) * 1.5);
  height: calc(1px + var(--titlebar-height));
  text-align: center;
  line-height: calc(var(--titlebar-height) - 1px);

  transition: background-color 0.3s ease-in;
}

.titlebar-minimize {
  align-items: flex-end;
}

.titlebar-minimize:hover,
.titlebar-close:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.titlebar-close {
  border-top-right-radius: 8px;
}

.titlebar-minimize svg,
.titlebar-resize svg.maximize-svg,
.titlebar-resize svg.fullscreen-svg,
.titlebar-close svg {
  width: calc(var(--titlebar-height) / 3);
  height: calc(var(--titlebar-height) / 3);
  shape-rendering: crispEdges;
}
</style>
