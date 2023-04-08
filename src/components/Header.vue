<script setup>
import {useConfigStore} from '../store/index.js'
import {writeBinaryFile} from '@tauri-apps/api/fs';
import {path, dialog} from '@tauri-apps/api';
import {ref, reactive} from 'vue'

const configStore = useConfigStore()
var loadJSONFile = null
const helpDialog = ref(false)


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
  <v-app-bar-title>
    <strong>
      aPaaS数据模拟
    </strong>
  </v-app-bar-title>
  <v-btn @click="handleFileUpload()">
    导入配置
    <v-icon end icon="mdi-file-upload-outline"></v-icon>
  </v-btn>
  <v-file-input
      @change="loadConfig()"
      accept=".json"
      v-model="loadJSONFile"
      v-show=false
      id="fileInput"></v-file-input>
  <v-btn @click="saveConfig()">
    导出配置
    <v-icon end icon="mdi-file-download-outline"></v-icon>
  </v-btn>


  <div class="text-center">
    <v-dialog
        v-model="helpDialog"
        width="auto"
    >
      <template v-slot:activator="{ props }">
        <v-btn
            v-bind="props"
            icon="mdi-information-slab-circle-outline">
        </v-btn>
      </template>
      <v-card>
        <v-card-text>
          <strong>
            📚使用手册参考:Slink
            数据模拟工具 https://ku.baidu-int.com/knowledge/HFVrC7hq1Q/LtinBUYBKY/FcCtZVI_f5/S1fyDXx0SkX-TP
            <br>
            <br>
            🤔其他问题联系 xuyanpeng01@baidu.com (๑•̀ㅂ•́)و✧
          </strong>
        </v-card-text>
        <v-card-actions>
          <v-btn block @click="helpDialog = false">确认</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>


  <div></div>
</template>

<style scoped></style>
