<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Layout from "./components/Layout.vue";
import ProgressBar from "./components/ProgressBar.vue";
import ServerStatus from "./components/ServerStatus.vue";
import Button from "./components/Button.vue";
import Addons from "./components/Addons.vue";
import UsersIcon from "./components/UsersIcon.vue";
import DropdownMenu from "./components/DropdownMenu.vue";
import AccountOutlineIcon from "./components/AccountOutlineIcon.vue";
import ChevronDown from "./components/ChevronDown.vue";
import Link from "./components/Link.vue";
import { Icon } from "@iconify/vue";

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

const statusRealms = ref<StatusRealm[]>([]);
const online = ref(0);
const viewState = ref<"news" | "addons">("news");

console.log("getVersion", getVersion);

function viewNews() {
  viewState.value = "news";
}

function viewAddons() {
  viewState.value = "addons";
}

onMounted(async () => {
  const status = await httpGet<StatusResponse>(
    "https://sirus.su/api/statistic/tooltip.json"
  );

  if (status) {
    statusRealms.value = status.realms;
    online.value = status.online_count;
  }

  version.value = await getVersion();
});
</script>

<template>
  <Layout>
    <template #header>
      <nav class="primary-nav">
        <Button variant="nav" @click="viewNews">Новости</Button>
        <Button variant="nav" @click="viewAddons">Аддоны</Button>
        <DropdownMenu open-on="hover">
          <Button variant="nav">О Игре</Button>
          <template #primary>
            <Button variant="nav" text="left" href="https://sirus.su/base/x5"
              >Поиск по базе</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/talents"
              >Калькулятор талантов</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/pet-talents"
              >Калькулятор талантов питомцев</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/addons"
              >Наши Аддоны</Button
            >
            <Button variant="nav" text="left" href="http://siruslogs.su/"
              >Логи Рейдов</Button
            >
          </template>
        </DropdownMenu>
      </nav>

      <aside class="aside-nav">
        <DropdownMenu open-on="hover">
          <Button variant="nav">Статы</Button>
          <template #primary>
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/ladder/bg/x5"
              >Рейтинги BG</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/ladder/arena/x5"
              >Рейтинги арены</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/ladder/pve/players/x5"
              >Рейтинги PVE</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/pve-progression/realm-progress/x5"
              >PVE Прогресс</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/statistic/online"
              >Игроки онлайн</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/statistic/changelog"
              >Список изменений</Button
            >
            <Button
              variant="nav"
              text="left"
              href="https://sirus.su/base/events/frozen-snowman-lair"
              >Замерзший снеговик</Button
            >
          </template>
        </DropdownMenu>
        <DropdownMenu open-on="hover">
          <Button variant="nav">
            <AccountOutlineIcon />&nbsp;Account <ChevronDown />
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

    <template v-if="viewState === 'addons'">
      <Addons />
    </template>

    <template #aside>
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
    <template #footer>
      <section class="footer-content">
        <div class="play-button">
          <Button class="play-button-run" variant="play"> Play </Button>
          <Button class="play-button-edit" variant="play">
            <Icon
              icon="mdi:settings"
              width="1.5rem"
              height="1.5rem"
              color="white"
            />
          </Button>
        </div>

        <ProgressBar />
        <div class="meta">Version: {{ version }}</div>
      </section>
    </template>
  </Layout>
</template>

<style scoped>
.play-button {
  position: relative;
  display: flex;
  flex-direction: row;
}

button.play-button-run {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  min-width: 200px;
}

.play-button-edit {
  border-left: 1px solid hsl(15, 100%, 40%);
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
  min-width: 40px;
  padding: 0 4px;
}

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
