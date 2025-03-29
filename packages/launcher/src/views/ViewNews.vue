<script setup lang="ts">
import { onMounted, ref } from "vue";
import CardList from "../components/CardList.vue";
import CardFull from "../components/CardFull.vue";
import { NewsItem, NewsResponse, httpGet } from "../api";

const items = ref<NewsResponse["data"]>([]);
const cardItem = ref<NewsItem>();

function showCard(item: NewsItem) {
  cardItem.value = item;
}

function hideCard() {
  cardItem.value = undefined;
}

onMounted(async () => {
  const news = await httpGet<NewsResponse>("https://api.sirus.su/api/news");

  if (news?.data) {
    items.value = news.data.splice(0, 3);
  }
});
</script>

<template>
  <section>
    <CardFull v-if="cardItem" :item="cardItem" @back="hideCard" />
    <CardList v-else :items="items" @show="showCard" />
  </section>
</template>
