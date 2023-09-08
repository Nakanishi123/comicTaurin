<script setup lang="ts">
import { useBook } from '../stores/Book';

const book = useBook();



function nextPage(page: number) {
    if (book.nowPageNum < book.pagePaths.length - 1) {
        book.nowPageNum += page;
    }
    else {
        console.log("no page");
    }
}

function prevPage() {
    if (book.nowPageNum > 0) {
        book.nowPageNum -= 2;
    } else {
        console.log("no page");
    }
}

function wheel(e: WheelEvent) {
    if (e.deltaY < 0) {
        prevPage();
    } else {
        nextPage(2);
    }
}
</script>


<template>
    <div id="image-container" @click="nextPage(1)" @contextmenu="prevPage()" @wheel="wheel">
        <img class="image" id="imgLeft" :src="book.page2" alt="">
        <img class="image" id="imgRight" :src="book.page1" alt="">
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