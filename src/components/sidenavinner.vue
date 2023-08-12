<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ref, watch } from "vue";
import { useNowPath } from "../stores/paths.ts"
import { ArrowUpIcon, FolderIcon } from '@heroicons/vue/24/solid'

const nowPath = useNowPath();
const { childData } = storeToRefs(nowPath)
const inputValue = ref("");
watch(nowPath, () => {
    inputValue.value = nowPath.nowPath;
})
</script>

<template>
    <div class="container">
        <div class="utils">
            <ArrowUpIcon class="icon" @click="nowPath.toParent()" />
        </div>
        <form onsubmit="return false;">
            <input v-model="inputValue" type="text" @keypress.enter="nowPath.selectPath(inputValue)">
        </form>

        <div class="list-path">
            <li v-for=" path in childData" @dblclick="nowPath.selectPath(path)">
                <div class="file-name">
                    {{ path.name }}
                </div>
                <FolderIcon class="icon" v-if="path.is_dir" />
            </li>
        </div>
    </div>
</template>

<style scoped>
.container {
    height: 100%;
    display: flex;
    flex-flow: column;
    padding: 10px;
    background-color: var(--background-color);
}

.utils .icon {
    width: 25px;
    height: 25px;
    color: var(--text-color);
    border: 2px solid transparent;
}

.utils .icon:hover {
    border-color: var(--text-color);
}

input {
    padding: 0;
    width: 100%;
    height: 28px;
    color: var(--text-color);
    background-color: var(--background);
    border: 2px solid var(--text-color);
    font-size: 20px;
}

.list-path {
    list-style-type: none;
    overflow-y: scroll;
    overflow-x: hidden;
    white-space: nowrap;
    margin: 10px 0 20px;
}

.list-path::-webkit-scrollbar {
    display: none;
}

.list-path li {
    display: flex;
    justify-content: space-between;
    list-style: none;
    padding: 2px;
    border: 2px solid transparent;
    color: var(--text-color);
}

.list-path li:hover {
    border-color: var(--text-color);
}

.list-path .icon {
    height: 25px;
}
</style>