<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const leftImage = ref<string | undefined>(undefined);
const rightImage = ref<string | undefined>(undefined);

const nextpage = async () => {
    const [left, right] = await Promise.all([invoke("get_randompic") as Promise<string>, invoke("get_randompic") as Promise<string>]);
    leftImage.value = left;
    rightImage.value = right;
};
document.oncontextmenu = function () { return false; }
</script>


<template>
    <div id="image-container" @click="nextpage" @contextmenu="nextpage" @wheel="nextpage">
        <img class="image" id="imgLeft" :src="leftImage" alt="">
        <img class="image" id="imgRight" :src="rightImage" alt="">
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