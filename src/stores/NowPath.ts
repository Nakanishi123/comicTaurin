import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { join } from '@tauri-apps/api/path';
import { FileEntry, exists, readDir } from '@tauri-apps/api/fs';

export const useNowPath = defineStore('nowPath', () => {
    const nowPath = ref<string>("");
    const childPaths = ref<Array<FileEntry>>();
    const pathBefore: string[] = [];
    const pathAfter: string[] = [];
    watch(nowPath, () => {
        try {
            readDir(nowPath.value).then((entries) => {
                childPaths.value = entries;
            });
        } catch (e) { }
    });

    function changeNowPath(newPath: string) {
        if (newPath.trim() == nowPath.value) {
            return;
        }

        exists(newPath.trim()).then((isExists) => {
            if (isExists) {
                pathBefore.push(nowPath.value);
                nowPath.value = newPath;
            } else {
                console.log('not exists', newPath);
            }
        });
    }

    function toParent() {
        join(nowPath.value, '..').then((path) => {
            changeNowPath(path);
        });
    }

    function toBeforePath() {
        if (pathBefore.length > 0) {
            pathAfter.push(nowPath.value);
            nowPath.value = pathBefore.pop()!;
        }
    }

    function toAfterPath() {
        if (pathAfter.length > 0) {
            changeNowPath(pathAfter.pop()!);
        }
    }

    changeNowPath("H:\\User\\picture\\manga");
    return {
        nowPath, childPaths, changeNowPath, toParent, toBeforePath, toAfterPath
    };
});