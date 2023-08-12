import { createApp } from "vue";
import "@fontsource/noto-sans-jp/500.css";
import "./styles.css";
import { createPinia } from 'pinia';
import App from "./App.vue";

const app = createApp(App);
app.use(createPinia());
app.mount('#app');