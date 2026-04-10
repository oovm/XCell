import ElementPlus from "element-plus";
import { createApp } from "vue";
import { createPinia } from "pinia";
import "element-plus/dist/index.css";
import "./style.css";
import App from "./App.vue";
import router from "./router";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(ElementPlus);
app.use(router);
app.mount("#app");
