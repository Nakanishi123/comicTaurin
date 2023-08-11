<script setup lang="ts">
</script>


<template>
    <div id="image-container">
        <img class="image" id="imgLeft" src="null" alt="">
        <img class="image" id="imgRight" src="null" alt="">
    </div>
</template>

<style scoped>
.image {
    width: 50vh;
    height: 100vh;
}

#image-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
}
</style>