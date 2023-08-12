import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { PathInfo } from "../components/rustStruct.ts"
import { invoke } from "@tauri-apps/api/tauri";

export const useNowPath = defineStore('nowPath', () => {
    const nowPath = ref("");
    const childData = ref<Array<PathInfo>>([]);

    async function getChild() {
        try {
            const response = await invoke("get_children", { dirPathStr: nowPath.value });
            childData.value = response as PathInfo[];
        } catch (error) {
            console.error("Error fetching children:", error);
        }
    };

    function selectPath(path: PathInfo | string): void {
        if (typeof path === "string") {
            nowPath.value = path;
        } else {
            if (path.is_dir) {
                nowPath.value = path.path;
            } else {

            }
        }
    }

    async function toParent(): Promise<void> {
        invoke<PathInfo>("get_parent", { pathStr: nowPath.value }).then(parent_path => {
            nowPath.value = parent_path.path;
            console.log("parent_path:", parent_path);
        });
    }

    watch(nowPath, () => {
        getChild();
    });

    return { nowPath, childData, selectPath, toParent };
});