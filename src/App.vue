<script setup lang="ts">
// import viewer from "./components/viewer.vue";
import BaseSidenav from "./components/Sidenav/BaseSidenav.vue";
import View from "./components/View.vue";

document.oncontextmenu = function () {
  return false;
}
</script>

<template>
  <View />
  <BaseSidenav />
</template>

<style scoped></style>
