import { createApp } from "vue";
import App from "./App.vue";
import init, { init_wasm } from "../vwbrs/pkg/vwbrs.js";


import "./assets/main.css";

init().then(function() {
    init_wasm();
})

createApp(App).mount("#app");
