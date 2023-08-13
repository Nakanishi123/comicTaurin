import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { PathInfo } from "../components/rustStruct.ts"
import { invoke } from "@tauri-apps/api/tauri";

export const useNowPath = defineStore('nowPath', () => {
    // パスの情報を保持する
    const nowPath = ref("");
    const childData = ref<Array<PathInfo>>([]);
    const bookOpen = ref(false);

    async function getChildren() {
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
                console.log(path.path);
                if (bookExts.includes(path.path.split('.').pop() as string)) {
                    // 画像の場合
                    invoke("read_zip_book", { bookPath: path.path, imageExts: imageExts }).then((response) => {
                        console.log(response);
                        nowPageNum.value = response as number;
                    });
                    bookOpen.value = true;
                }
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
        getChildren();
    });

    // 本の情報
    const imageExts = ["png", "jpg", "jpeg", "gif", "bmp", "webp", "avif"];
    const bookExts = ["zip", "cbz"];
    const nowPageNum = ref(0);
    const nowPage = ref(1);
    const nowPageImage = ref("");

    async function getPage(pageNum: number) {
        console.log("getPage:", pageNum);
        invoke("get_page", { pageIndex: pageNum }).then((response) => {
            nowPageImage.value = response as string;
        });
    }

    function nextPage(): void {
        if (nowPage.value < nowPageNum.value) {
            nowPage.value += 1;
        }
    }

    function prevPage(): void {
        if (nowPage.value > 1) {
            nowPage.value -= 1;
        }
    }
    watch(nowPage, () => {
        getPage(nowPage.value);
    });

    return { nowPath, childData, selectPath, toParent, nowPage, nowPageImage, nextPage, prevPage };
});