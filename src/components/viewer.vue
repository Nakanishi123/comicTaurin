<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useNowPath } from "../stores/paths.ts"

const nowPath = useNowPath();
const { nowPageImage } = storeToRefs(nowPath);
document.oncontextmenu = function () { return false; }
</script>


<template>
    <div id="image-container" @click="nowPath.nextPage()" @contextmenu="nowPath.prevPage()" @wheel="nowPath.nextPage()">
        <img class="image" id="imgLeft" :src="nowPageImage" alt="">
        <img class="image" id="imgRight" :src="nowPageImage" alt="">
    </div>
</template>

<style scoped>
#image-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
}

.image {
    max-width: 50%;
    max-height: 100%;
}
</style>