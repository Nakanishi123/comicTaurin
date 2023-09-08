import { defineStore } from 'pinia';
import { useNowPath } from './NowPath';
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import natsort from 'natsort';

export const useBook = defineStore('book', () => {
    const comicExts = ["cbz", "zip"]
    const nowPath = useNowPath();
    const pagePaths = ref<string[]>([]);
    const book = ref<string>("");

    const nowPageNum = ref<number>(0);
    const page1 = ref<string>("");
    const page2 = ref<string>("");

    async function setBook(path: string) {
        book.value = path;
        pagePaths.value = await (invoke('get_page_paths', { pathStr: path })) as string[];
        pagePaths.value = pagePaths.value.sort(natsort());
    }

    async function getPage(pageNum: number) {
        if (pageNum < 0 || pageNum >= pagePaths.value.length) {
            return "";
        }

        return await invoke('get_page', { pathStr: book.value, page: pagePaths.value[pageNum] }) as string;
    }


    watch(nowPath, () => {
        comicExts.forEach(async ext => {
            if (nowPath.nowPath.endsWith(ext)) {
                setBook(nowPath.nowPath);
            }
        });
    });

    watch(nowPageNum, async () => {
        const [result1, result2] = await Promise.all([
            getPage(nowPageNum.value),
            getPage(nowPageNum.value + 1)
        ]);

        page1.value = result1;
        page2.value = result2;
    });

    return {
        book, pagePaths, nowPageNum, page1, page2, getPage
    };
});