<script setup lang="ts">
import { useNowPath } from "../../stores/NowPath.ts"
import ArrowLeft from 'vue-material-design-icons/ArrowLeft.vue';
import ArrowRight from 'vue-material-design-icons/ArrowRight.vue';
import ArrowUp from 'vue-material-design-icons/ArrowUp.vue';
import Home from 'vue-material-design-icons/Home.vue';

const nowPath = useNowPath();
</script>

<template>
    <div class="icon-container">
        <Home class="icon" @click="nowPath.toParent" />
        <ArrowUp class="icon" @click="nowPath.toParent" />
        <ArrowLeft class="icon" @click="nowPath.toBeforePath" />
        <ArrowRight class="icon" @click="nowPath.toAfterPath" />
    </div>
</template>

<style scoped>
.material-design-icon {
    color: var(--text-color);
}

.material-design-icon:hover {
    color: var(--text-accent-color);
}

.material-design-icon:active {
    color: var(--text-accent-color2);
}
</style>