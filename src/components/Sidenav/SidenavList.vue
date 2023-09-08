<script setup lang="ts">
import { useNowPath } from "../../stores/NowPath.ts"
import FolderOutline from 'vue-material-design-icons/FolderOutline.vue';
const nowPath = useNowPath();

</script>

<template>
    <ul class="list-path">
        <li v-for="childPath in nowPath.childPaths" :key="childPath.path" class="list-item"
            @dblclick="nowPath.changeNowPath(childPath.path)">
            <span class="text">{{ childPath.name }}</span>
            <FolderOutline v-if="childPath.children" class="folder-icon" />
        </li>
    </ul>
</template>

<style scoped>
.list-path {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
}

.list-path::-webkit-scrollbar {
    display: none;
}

ul {
    padding: 0;
    margin: 10px 0 0;
}

.list-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--text-color);
    white-space: nowrap;
    text-overflow: ellipsis;
    border: solid 1px var(--background-color);
    padding: 0px 5px;
}

.list-item:hover {
    border: solid 1px var(--text-accent-color);
}

.text {
    flex: 1;
}

.folder-icon {
    height: 100%;
    align-content: center;
}
</style>