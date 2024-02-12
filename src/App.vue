<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CardList from "./components/CardList.vue";
import CardFull from "./components/CardFull.vue";
import Layout from "./components/Layout.vue";
import ProgressBar from "./components/ProgressBar.vue";
import ServerStatus from "./components/ServerStatus.vue";
import Button from "./components/Button.vue";
import UsersIcon from "./components/UsersIcon.vue";
import DropdownMenu from "./components/DropdownMenu.vue";
import AccountOutline from "./components/AccountOutline.vue";
import ChevronDown from "./components/ChevronDown.vue";
import Link from "./components/Link.vue";

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
const online = ref(0);

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
    "https://sirus.su/api/statistic/tooltip.json"
  );

  if (status) {
    statusRealms.value = status.data.realms;
    online.value = status.data.online_count;
  }

  const newsResponse = await httpGet<NewsResponse>(
    "https://api.sirus.su/api/news"
  );

  if (newsResponse?.data?.data) {
    items.value = newsResponse.data.data.splice(0, 3);
  }

  version.value = await getVersion();

  console.log("response", newsResponse);
});
</script>

<template>
  <Layout>
    <template v-slot:header>
      <na class="primary-nav">
        <Button variant="nav" href="#">Community</Button>
        <Button variant="nav" href="#">News</Button>
        <DropdownMenu open-on="hover">
          <Button variant="nav"> Game Info </Button>
          <template #primary>
            <Button variant="nav" text="left">Search</Button>
            <Button variant="nav" text="left">Talent Calculator</Button>
            <Button variant="nav" text="left">Addons</Button>
            <Button variant="nav" text="left">Raid logs</Button>
          </template>
        </DropdownMenu>
      </na>

      <aside class="aside-nav">
        <DropdownMenu open-on="hover">
          <Button variant="nav">Stats</Button>
          <template #primary>
            <Button variant="nav" text="left">Arena rating</Button>
            <Button variant="nav" text="left">BG rating</Button>
            <Button variant="nav" text="left">PVE rating</Button>
            <Button variant="nav" text="left">PVE progress</Button>
            <Button variant="nav" text="left">Players online</Button>
            <Button variant="nav" text="left">Changelog</Button>
            <Button variant="nav" text="left">Snowman</Button>
          </template>
        </DropdownMenu>
        <DropdownMenu open-on="hover">
          <Button variant="nav">
            <AccountOutline /> Account <ChevronDown />
          </Button>
          <template #primary>
            <div class="dropdown-login">
              <Button variant="login">Login</Button>
            </div>
            <Button variant="nav" text="left">Profile</Button>
            <Button variant="nav" text="left">Account Settings</Button>
            <Button variant="nav" text="left">Signup</Button>
          </template>
          <template #secondary>
            <Button variant="nav" text="left">Support</Button>
            <Button variant="nav" text="left">Donate!</Button>
          </template>
        </DropdownMenu>
      </aside>
    </template>

    <CardFull v-if="cardItem" :item="cardItem" @back="hideCard" />
    <CardList v-else :items="items" @show="showCard" />

    <template v-slot:aside>
      <div id="logo"></div>
      <div class="users-online">
        <UsersIcon /> {{ online }} <span class="users-online-meta">online</span>
      </div>

      <div class="main-links">
        <Link class="main-links-donate" href="https://sirus.su/pay"
          >Donate!</Link
        >
        <Link href="https://sirus.su/vote">Vote</Link>
        <Link href="https://forum.sirus.su/">Forums</Link>
        <Link href="https://forum.sirus.su/threads/39324/"
          >Connection problems ?</Link
        >
        <Link href="https://forum.sirus.su/threads/39324/">Profile</Link>
      </div>

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
.primary-nav,
.aside-nav {
  display: flex;
  flex-direction: row;
}

.dropdown-login {
  padding: 16px 12px;
}

.dropdown-links {
  justify-content: flex-start;
}

.users-online {
  color: white;
  font-size: 18px;
  font-weight: 700;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}

.users-online-meta {
  font-weight: 400;
  font-size: 12px;
}

.main-links {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 4px;
}

.main-links a {
  font-size: 14px;
  color: #ebdec2;
  text-decoration: none;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.main-links a:hover {
  opacity: 0.9;
}

a.main-links-donate {
  color: #f8b700;
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
  width: 170px;
  height: 70px;
  background: url("./assets/logo.png") no-repeat;
  background-size: contain;
  background-position: center;
  align-self: center;
}
</style>
