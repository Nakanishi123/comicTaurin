<script setup lang="ts">
import { ref } from 'vue';
import SidenavContent from './SidenavContent.vue';
const sidebarWidth = ref(250);
const isResizing = ref(false);
const isHidden = ref(false);
const isHiddenTimeout = ref(false);


const startResize = (e: MouseEvent) => {
    e.preventDefault();
    isResizing.value = true;
    document.addEventListener("mousemove", resize);
    document.addEventListener("mouseup", stopResize);
}
const resize = (e: MouseEvent) => {
    isHidden.value = false;
    isHiddenTimeout.value = false;
    if (!isResizing.value) return;

    const x = e.pageX;
    if (x < 50 || x > document.body.clientWidth - 50) {
        return;
    }
    sidebarWidth.value = x;
}
const stopResize = () => {
    isResizing.value = false;
    document.removeEventListener("mousemove", resize);
    document.removeEventListener("mouseup", stopResize);
}

// サイドバーを隠す．マウスがサイドバーの上にあるときは隠さない

document.addEventListener("mousemove", (e) => {
    if (e.pageX < 50) {
        isHidden.value = false;
        isHiddenTimeout.value = false;
    }
});
const sidenavLeave = () => {
    isHiddenTimeout.value = true;
    setTimeout(() => {
        if (isHiddenTimeout.value) {
            isHidden.value = true;
        }
    }, 1000);
}
const sidenavOn = () => {
    isHidden.value = false;
    isHiddenTimeout.value = false;
}
</script>
  
<template>
    <div id="sidenav" :style="{ width: sidebarWidth + 'px' }" @mouseleave="sidenavLeave" @mousemove="sidenavOn"
        :hidden="isHidden">
        <SidenavContent />
        <div id="splitter" @mousedown="startResize" :style="{ left: sidebarWidth + 'px' }"></div>
    </div>
</template>

<style scoped>
#sidenav {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
}

#splitter {
    position: absolute;
    top: 0;
    width: 3px;
    height: 100%;
    background-color: var(--text-color);
    cursor: ew-resize;
}
</style>
