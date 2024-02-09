<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Titlebar from "./components/Titlebar.vue";
import PlayIcon from "./components/PlayIcon.vue";
import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
import { fetch } from "@tauri-apps/api/http";

type NewsResponse = {
  data: Array<{
    title: string;
    id: number;
    description: string;
    cover_url: string;
    category: {
      name: string;
      id: number;
    };
  }>;
};

type NewsItem = NewsResponse["data"][0];

const version = ref("0.0.0");
const items = ref<NewsResponse["data"]>([]);
const cardItem = ref<NewsItem>();

console.log("getVersion", getVersion);

function httpGet<T>(url: string) {
  return fetch<T>(url, {
    method: "GET",
    timeout: 30,
    headers: {
      "User-Agent": `sirus-launcher 1.3.1`,
    },
  });
}

function getCoverUrl(url: string) {
  return `https://api.sirus.su${url}`;
}

function showCard(item: NewsItem) {
  cardItem.value = item;
}

onMounted(async () => {
  // version.value = await getVersion();
  console.log(version.value);

  const response = await httpGet("https://api.sirus.su/api/server/status");
  const newsResponse = await httpGet<NewsResponse>(
    "https://api.sirus.su/api/news"
  );

  if (newsResponse?.data?.data) {
    items.value = newsResponse.data.data.splice(0, 3);
  }

  console.log("response", response);
  console.log("response", newsResponse);
});
</script>

<template>
  <div id="app" data-tauri-drag-region>
    <Titlebar />
    <div id="container">
      <header>
        <nav>
          <a href="#">Community</a>
          <a href="#">News</a>
        </nav>

        <aside>
          <button>Account</button>
        </aside>
      </header>
      <aside>
        <div id="logo"></div>
        <div class="online">15897 online</div>

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
      </aside>

      <main>
        <div v-if="cardItem" class="card-full-container">
          <div class="card-full">
            <div class="card-full-image">
              <img :src="getCoverUrl(cardItem.cover_url)" alt="" />
            </div>
            <div class="card-full-category">{{ cardItem.category.name }}</div>
            <div class="card-full-title">{{ cardItem.title }}</div>
            <div class="card-full-text" v-html="cardItem.description"></div>
            <div class="card-full-actions">
              <a href="#">Discuss</a>
              <button @click="cardItem = undefined">Back</button>
            </div>
          </div>
        </div>

        <div v-else class="card-list">
          <div
            class="card"
            :key="item.id"
            v-for="item in items"
            @click="showCard(item)"
          >
            <div class="card-image">
              <img :src="getCoverUrl(item.cover_url)" alt="" />
            </div>
            <div class="card-meta">
              <div class="card-category">{{ item.category.name }}</div>
              <div class="card-title">
                {{ item.title }}
              </div>
            </div>
          </div>
        </div>
      </main>

      <footer>
        <section class="footer-content">
          <button id="play">Play</button>
          <div class="progress">
            <div class="progress-content">
              <div class="progress-bar">
                <div class="progress-bar-line"></div>
              </div>
              <button class="progress-action"><PlayIcon /></button>
            </div>
            <div class="progress-meta">
              <span>99% (25 GB)</span> <span>2.5 MB/s</span>
            </div>
          </div>
          <div class="meta">Version: {{ version }}</div>
        </section>
      </footer>
    </div>
  </div>
</template>

<style scoped>
.card-full-container {
  max-height: 500px;
  overflow-y: scroll;
  overflow-x: hidden;
  padding: 8px;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
}

.card-full {
  display: flex;
  width: 1000px;
  flex-direction: column;
}

.card-full-title {
  color: white;
  font-family: Semplicita Pro, Open Sans, Arial, Helvetica, sans-serif;
  font-size: 52px;
  font-weight: 700;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-full-category {
  margin-top: 12px;
  font-size: 18px;
  color: white;
  text-transform: uppercase;
  font-weight: 400;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}
.card-full-text {
  line-height: 1.5;
  color: white;
  font-size: 18px;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.card-full-image {
  width: 100%;
  border: 1px solid rgba(0, 0, 0, 0.2);
  height: 320px;
  border-radius: 8px;
  overflow: hidden;
}
.card-full-image img {
  object-fit: cover;
  width: 100%;
  height: 100%;
}

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

.online {
  color: white;
  font-size: 14px;
  text-align: center;
}
.meta {
  font-size: 12px;
}

.footer-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

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

#play {
  background: #aa2409;
  height: 56px;
  font-size: 24px;
  font-weight: 500;
  font-family: Object Sans, sans-serif;
  padding: 0 40px;
  border-radius: 4px;
  min-width: 240px;
}

#play:hover {
  border-color: #b23300;
}

div#logo {
  display: block;
  width: 240px;
  height: 140px;
  background: url("./assets/logo.png") no-repeat;
  background-size: contain;
  background-position: center;
}

header {
  background: rgba(60, 42, 41, 0.8);
  padding: 12px;
  border-radius: 8px;
}

header button {
  font-size: 18px;
  font-weight: 600;
  background: none;
  border-radius: 8px;
  min-width: 120px;
}

header button:hover {
  background: rgba(255, 255, 255, 0.05);
}

header a {
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

header a:hover {
  background: rgba(255, 255, 255, 0.05);
}

div#app {
  user-select: none;
  position: relative;
  padding: 24px;
  border-radius: 12px;
  background: url("https://wow.zamimg.com/uploads/screenshots/normal/1059910.jpg")
    no-repeat;
  background-size: cover;
  padding-top: 54px;
  padding-bottom: 12px;
  border: 1px solid #44454c;
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

.card-list {
  display: flex;
  flex-direction: row;
  gap: 12px;
  flex-wrap: wrap;
}

.card {
  border-radius: 8px;
  overflow: hidden;
  width: 340px;
  height: 340px;
  display: flex;
  flex-direction: column;
  justify-content: stretch;
}

.card-image {
  border-radius: 8px 8px 0 0;
  overflow: hidden;
  flex-grow: 1;
}

.card-image img {
  object-fit: cover;
  width: 100%;
  height: 100%;
}

.card-category {
  font-size: 12px;
}

.card-meta {
  /* background: #22252b; */
  background: rgba(0, 0, 0, 0.8);
  padding: 20px;
  color: white;
  height: 120px;
}
</style>
