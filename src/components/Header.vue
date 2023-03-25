<script setup>
import { useConfigStore } from '../store/index.js'
const configStore = useConfigStore()
var JSONFile = null
var JSONContent = null


function handleFileUpload() {
    var input = document.getElementById("fileInput")
    input.click()
}

function importJSON() {
    if (!JSONFile) { console.log("No File Chosen") }
    var reader = new FileReader()

    reader.readAsText(document.getElementById("fileInput").files[0])
    reader.onload = () => {
        configStore.config = JSON.parse(reader.result)
        console.log("加载配置")
        console.log(configStore.config)
    }
}

</script>

<template>
    <v-app-bar color="teal-darken-4">
        <v-app-bar-title style="font-weight: bolder;">aPaaS数据模拟</v-app-bar-title>
        <v-btn @click="handleFileUpload()">
            导入配置
        </v-btn>
        <v-file-input @change="importJSON()" accept=".json" v-model="JSONFile" v-show=false id="fileInput"></v-file-input>
        <v-btn>
            导出配置
        </v-btn>
    </v-app-bar>
    <div></div>
</template>

<style scoped></style>
