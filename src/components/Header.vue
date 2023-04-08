<script setup>
import {useConfigStore} from '../store/index.js'
import {writeBinaryFile} from '@tauri-apps/api/fs';
import {path, dialog} from '@tauri-apps/api';

const configStore = useConfigStore()
var loadJSONFile = null


function handleFileUpload() {
  var input = document.getElementById("fileInput")
  input.click()
}

function loadConfig() {
  if (!loadJSONFile) {
    console.log("No File Chosen")
  }
  var reader = new FileReader()

  reader.readAsText(document.getElementById("fileInput").files[0])
  reader.onload = () => {
    configStore.config = JSON.parse(reader.result)
    console.log("加载配置")
    console.log(configStore.config)
  }
}

async function saveConfig() {
  let conf = JSON.stringify(configStore.config);
  const basePath = await path.downloadDir();
  let selPath = await dialog.save({
    defaultPath: basePath,
    filters: [{name: 'config', extensions: ['json']}]

  });
  console.log(selPath)
  const reader = new FileReader();
  reader.readAsArrayBuffer(new Blob([conf], {type: 'text/plain'}));
  reader.onload = function (e) {
    let fileU8A = new Uint8Array(e.target.result);
    writeBinaryFile({contents: fileU8A, path: `${selPath}`});
  };
}
</script>

<template>
  <v-app-bar-title>aPaaS数据模拟</v-app-bar-title>
  <v-btn @click="handleFileUpload()">
    导入配置
  </v-btn>
  <v-file-input
      @change="loadConfig()"
      accept=".json"
      v-model="loadJSONFile"
      v-show=false
      id="fileInput"></v-file-input>
  <v-btn @click="saveConfig()">
    导出配置
  </v-btn>

  <div></div>
</template>

<style scoped></style>
