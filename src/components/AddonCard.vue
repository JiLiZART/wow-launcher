<script setup lang="ts">
const angle = 135;

const palettes = [
  {
    from: "#f40076",
    to: "#df98fa",
  },
  {
    from: "#f06966",
    to: "#fad6a6",
  },
  {
    from: "#ff0076",
    to: "#590fb7",
  },
  {
    from: "#9055ff",
    to: "#13e2da",
  },
  {
    from: "#0b63f6",
    to: "#003cc5",
  },
  {
    from: "#d6ff7f",
    to: "#00b3cc",
  },
  {
    from: "#e233ff",
    to: "#ff6b00",
  },
  {
    from: "#df98fa",
    to: "#9055ff",
  },
  {
    from: "#ed7b84",
    to: "#9055ff",
  },
  {
    from: "#402565",
    to: "#30be96",
  },
  {
    from: "#402662",
    to: "#3900a6",
  },
  {
    from: "#f14658",
    to: "#dc2537",
  },
  {
    from: "#f40076",
    to: "#342711",
  },
  {
    from: "#000066",
    to: "#6699ff",
  },
  {
    from: "#cb5eee",
    to: "#4be1ec",
  },
  {
    from: "#fa7cbb",
    to: "#f14658",
  },
  {
    from: "#737dfe",
    to: "#ffcac9",
  },
  {
    from: "#2f80ed",
    to: "#b2ffda",
  },
];
const generateGradient = (text: string) => {
  const len = text.length;
  const plen = palettes.length;
  const item = palettes[len % plen];

  return `linear-gradient(${angle}deg, ${item.from}, ${item.to})`;
};

import Button from "./Button.vue";
import { Icon } from "@iconify/vue";

function timeSince(date: string) {
  var seconds = Math.floor(
    (new Date().getTime() - new Date(date).getTime()) / 1000
  );

  var interval = seconds / 31536000;

  if (interval > 1) {
    return Math.floor(interval) + " years";
  }
  interval = seconds / 2592000;
  if (interval > 1) {
    return Math.floor(interval) + " months";
  }
  interval = seconds / 86400;
  if (interval > 1) {
    return Math.floor(interval) + " days";
  }
  interval = seconds / 3600;
  if (interval > 1) {
    return Math.floor(interval) + " hours";
  }
  interval = seconds / 60;
  if (interval > 1) {
    return Math.floor(interval) + " minutes";
  }
  return Math.floor(seconds) + " seconds";
}

function kFormatter(num: number) {
  const sign = Math.sign(num);

  if (Math.abs(num) > 999) {
    const val = parseFloat((Math.abs(num) / 1000).toFixed(1));

    return `${sign * val}K`;
  }

  return String(sign * Math.abs(num));
}

function versionFormatter(version: string) {
  const parts = version.split(".");
  const [major, minor] = parts;

  if (parts.length === 1) {
    return `${major}.0`;
  }

  if (parts.length === 2) {
    return `${major}.${minor}.0`;
  }

  return version;
}

const { item, isInstalled, isInstalling, isRemoving } = defineProps<{
  isInstalled?: boolean;
  isInstalling?: boolean;
  isRemoving?: boolean;
  item: {
    name: string;
    version: string;
    downloads: number;
    repo_updated_at: string;
  };
}>();
</script>

<template>
  <div class="addon-card">
    <div
      class="addon-card-header"
      :style="{ background: generateGradient(item.name) }"
    >
      <div class="addon-card-title">
        {{ item.name }}
        <Icon
          icon="bitcoin-icons:verify-filled"
          width="1.5rem"
          height="1.5rem"
          color="#6cdb00"
        />
      </div>
      <div class="addon-card-version">
        ver
        <span class="addon-card-bigger">
          {{ versionFormatter(item.version) }}
        </span>
      </div>
    </div>

    <div class="addon-card-desc">
      <p>
        <strong>RaidRoll</strong> - это модификация, которую можно использовать
        для распределения добычи в рейде. В списке перечислены все игроки с
        идентификатором рядом с их именем. Затем он выполняет бросок и
        объявляет, кто является победителем (на основании идентификатора). Вы
        также можете отслеживать броски, сделанные игроками на любой выпавшей
        добыче. Эта функция поддерживает систему распределения добычи EPGP.
        Аддон также содержит трекер лута, который можно использовать для
        объявления или отслеживания выпавшего лута.
      </p>
      \n
      <p>
        <em
          >Спасибо игроку
          <a href="https://forum.sirus.su/members/17os86.175882/">Os</a>
          за перевод описания на Русский Язык.</em
        >
      </p>
    </div>

    <div class="addon-card-body">
      <div class="addon-card-meta">
        <div class="addon-card-downloads">
          <span class="addon-card-bigger">{{
            kFormatter(item.downloads)
          }}</span>
          downloads
        </div>
        <div class="addon-card-updated">
          {{ timeSince(item.repo_updated_at) }} ago
        </div>
      </div>
      <div class="addon-card-actions">
        <div class="addon-card-status">
          <div v-if="isInstalled" class="addon-card-status-text">
            <Icon icon="ic:round-check" width="1rem" height="1rem" /> Installed
          </div>
          <div v-if="isInstalling" class="addon-card-status-text">
            <Icon icon="line-md:loading-loop" width="1rem" height="1rem" />
            Installing addon
          </div>
          <div v-if="isRemoving" class="addon-card-status-text">
            <Icon icon="line-md:loading-loop" width="1rem" height="1rem" />
            Removing addon
          </div>
        </div>
        <Button v-if="!isInstalled" variant="install" :disabled="isInstalling"
          >Install</Button
        >
        <Button v-if="isInstalled" variant="remove">Remove</Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.addon-card {
  background: rgba(49, 51, 57, 1);
  border-radius: 8px;
  min-height: 180px;
  min-width: 280px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.addon-card-header {
  position: relative;
  padding: 16px;
  border-radius: 8px 8px 0 0;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  min-height: 80px;
}

.addon-card-header::after {
  content: "";
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  border-radius: 8px 8px 0 0;
  background: rgba(49, 51, 57, 0.2);
  background: linear-gradient(
    180deg,
    rgba(49, 51, 57, 0.6) 12%,
    rgba(255, 255, 255, 0) 96%
  );
  z-index: 1;
}

.addon-card-title {
  z-index: 2;
  color: white;
  font-size: 18px;
  font-weight: 700;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
  display: inline-flex;
  gap: 4px;
}

.addon-card-updated {
  z-index: 2;
  color: white;
  font-size: 12px;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.addon-card-body {
  padding: 16px;
  border-radius: 0 0 8px 8px;
  align-self: flex-end;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  justify-content: space-between;
}

.addon-card-meta {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}

.addon-card-version,
.addon-card-status,
.addon-card-downloads {
  z-index: 2;
  color: white;
  font-size: 12px;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.addon-card-status {
  display: flex;
  align-items: center;
  gap: 4px;
}

.addon-card-status-text {
  display: flex;
  align-items: center;
  gap: 4px;
}

.addon-card-bigger {
  font-size: 16px;
}

.addon-card-actions {
  width: 100%;
  display: flex;
  flex-direction: row;
  gap: 4px;
  align-items: center;
  justify-content: space-between;
}

.addon-card-desc {
  display: none;
}
</style>
