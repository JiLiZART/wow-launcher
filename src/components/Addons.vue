<script setup lang="ts">
import AddonCard from "./AddonCard.vue";

const items = [
  {
    id: 1,
    name: "Bartender4",
    version: "4",
    category: 18,
    downloads: 308630,
    repo_updated_at: "2023-09-02 22:11:03",
    repository: "sirus-addons\/Bartender4",
  },
  {
    id: 3,
    name: "TipTac",
    version: "3",
    category: 21,
    downloads: 72387,
    repo_updated_at: "2023-11-11 11:11:15",
    repository: "sirus-addons\/TipTac",
  },
  {
    id: 4,
    name: "MoveAnything",
    version: "11",
    category: 17,
    downloads: 170789,
    repo_updated_at: "2024-01-05 10:19:23",
    repository: "sirus-addons\/MoveAnything",
  },
  {
    id: 7,
    name: "DrDamage",
    version: "3",
    category: 20,
    downloads: 241227,
    repo_updated_at: "2023-09-02 22:12:13",
    repository: "sirus-addons\/DrDamage",
  },
  {
    id: 8,
    name: "RaidRoll",
    version: "2",
    category: 14,
    downloads: 87974,
    repo_updated_at: "2021-05-21 18:38:14",
    repository: "sirus-addons\/RaidRoll",
  },
];
</script>

<template>
  <div class="addons-list">
    <div class="addons-category">
      <div class="addons-category-title">Addons Category</div>
      <div class="addons-category-list">
        <AddonCard
          v-for="item in items"
          :item="item"
          :is-removing="item.id === 4"
          :is-installing="item.id === 7"
          :is-installed="item.id === 8"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.addons-list {
  max-height: 500px;
  overflow-y: scroll;
  overflow-x: hidden;
  /* padding: 8px; */
  border-radius: 8px;

  display: flex;
  flex-direction: column;
  gap: 16px;
}

.addons-category {
  padding: 8px;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.addons-category-title {
  padding: 0 16px;
  color: white;
  font-size: 16px;
  font-weight: 700;
  text-shadow: 0 0 1px transparent, 0 1px 2px rgba(0, 0, 0, 0.8);
}

.addons-category-list {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 12px;
}
</style>
