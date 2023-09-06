import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from 'element-plus' // Element Plus bileşenlerini kullanabilmek için eklendi
import 'element-plus/dist/index.css' // // Element Plus stilleri için eklendi
import axios from 'axios' // axios kullanabilmek(http requestlerini yapabilmek) için eklendi
import VueAxios from 'vue-axios' // vue tarafında axios entegrasyonu için eklendi

// Uygulamada ElementPlus paketini kullanabilmek için use ile bildirim yapıldı
// axios paketlerinin uygulamaya enjekte edilmesi için gerekli use bildirimi yapıldı
createApp(App).use(ElementPlus).use(VueAxios, axios).mount('#app')
