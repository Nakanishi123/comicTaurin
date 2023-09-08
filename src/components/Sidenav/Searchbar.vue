<script setup lang="ts">
import { useNowPath } from "../../stores/NowPath.ts"
const { changeNowPath } = useNowPath();

function changePath(payload: Event) {
    changeNowPath((payload.target as HTMLInputElement).value);
}
</script>

<template>
    <input type="text" v-on:keydown.enter="changePath" />
</template>
