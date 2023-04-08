import {createApp} from 'vue'
import App from './App.vue'
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import {createVuetify} from 'vuetify'
import * as components from 'vuetify/components'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as directives from 'vuetify/directives'
import {aliases, mdi} from 'vuetify/iconsets/mdi'
import * as blueprints from 'vuetify/blueprints'
import {createPinia} from 'pinia'

const pinia = createPinia()

const myCustomLightTheme = {
    dark: true,
    colors: {
        primary: '#354cbe',
        'primary-darken-1': '#316fb6',
        secondary: '#03DAC6',
        'secondary-darken-1': '#018786',
        error: '#B00020',
        info: '#2196F3',
        success: '#4CAF50',
        warning: '#FB8C00',
    }
}

const vuetify = createVuetify({
    components,
    directives,
    blueprints,
    ssr: true,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi,
        }
    },
    theme: {
        defaultTheme: 'myCustomLightTheme',
        themes: {
            myCustomLightTheme,
        }
    }
})
const app = createApp(App)
app.use(vuetify)
app.use(ElementPlus)
app.use(pinia)
app.mount('#app')