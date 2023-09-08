<script setup lang="ts">
import SidenavNav from './SidenavNav.vue';
import Searchbar from './Searchbar.vue';
import SidenavList from './SidenavList.vue';
import { useNowPath } from "../../stores/NowPath.ts"
const nowPath = useNowPath();
</script>

<template>
    <div class="container">
        <div class="now">
            {{ nowPath.nowPath || '&nbsp;' }}
        </div>
        <SidenavNav />
        <Searchbar />
        <SidenavList />
    </div>
</template>

<style scoped>
.now {
    color: var(--text-color);
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    direction: rtl;
    text-align: left;
}

.container {
    height: 100%;
    display: flex;
    flex-flow: column;
    padding: 10px 5px 100px 10px;
    background-color: var(--background-color);
    overflow: hidden;
}
</style>