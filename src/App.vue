<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CardList from "./components/CardList.vue";
import CardFull from "./components/CardFull.vue";
import Layout from "./components/Layout.vue";
import ProgressBar from "./components/ProgressBar.vue";
import ServerStatus from "./components/ServerStatus.vue";
import Button from "./components/Button.vue";

import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
import {
  NewsItem,
  NewsResponse,
  StatusResponse,
  StatusRealm,
  httpGet,
} from "./api";

const version = ref("0.0.0");
const items = ref<NewsResponse["data"]>([]);
const cardItem = ref<NewsItem>();
const statusRealms = ref<StatusRealm[]>([]);

console.log("getVersion", getVersion);

function showCard(item: NewsItem) {
  cardItem.value = item;
}

function hideCard() {
  cardItem.value = undefined;
}

onMounted(async () => {
  // version.value = await getVersion();
  console.log(version.value);

  const status = await httpGet<StatusResponse>(
    "https://sirus.su/api/sirus?lang=ru"
  );

  if (status) {
    statusRealms.value = status.data.realms;
    version.value = status.data.version;
  }

  const newsResponse = await httpGet<NewsResponse>(
    "https://api.sirus.su/api/news"
  );

  if (newsResponse?.data?.data) {
    items.value = newsResponse.data.data.splice(0, 3);
  }

  console.log("response", newsResponse);
});
</script>

<template>
  <Layout>
    <template v-slot:header>
      <nav>
        <Button variant="nav" href="#">Community</Button>
        <Button variant="nav" href="#">News</Button>
      </nav>

      <aside>
        <Button variant="nav">Account</Button>
      </aside>
    </template>

    <CardFull v-if="cardItem" :item="cardItem" @back="hideCard" />
    <CardList v-else :items="items" @show="showCard" />

    <template v-slot:aside>
      <div id="logo"></div>
      <div class="online">15897 online</div>

      <ServerStatus :items="statusRealms" />
    </template>
    <template v-slot:footer>
      <section class="footer-content">
        <Button variant="play">Play</Button>
        <ProgressBar />
        <div class="meta">Version: {{ version }}</div>
      </section>
    </template>
  </Layout>
</template>

<style scoped>
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
</style>
