<script setup>
import {useConfigStore} from '../store/index.js'
import {invoke} from '@tauri-apps/api'
import {listen} from '@tauri-apps/api/event'
import {ref, onMounted} from 'vue'
// 样式控制
const connecting = ref(false);
const infoBarShow = ref(false);
const infoBarText = ref("");
const log = ref("");
const running = ref(false);

// 核心数据
const configStore = ref(useConfigStore())

const runningMode = ref("current")
const timeInterval = ref(1000)
const timeScope = ref(["2023-04-12T16:00:00.000Z", "2023-05-22T16:00:00.000Z"])
const time_step = ref(60000)
const progress = ref(0)

function run() {
  console.log(JSON.stringify(configStore.value.config.modelConfs))

  invoke('run', {
    running_mode: runningMode.value,
    time_conf: {
      time_interval: Number(timeInterval.value),
      start_time: Date.parse(timeScope.value[0]),
      end_time: Date.parse(timeScope.value[1]),
      time_step: Number(time_step.value),
    },
    backend_end_point: configStore.value.config.common.backend_endpoint,
    model_confs_json_str: JSON.stringify(configStore.value.config.modelConfs)
  }).then((response) => {
    running.value = true
  })
}

function stop() {
  invoke('stop').then((response) => {
    running.value = false
  })
}

onMounted(() => {
  onListenLog()
  onListenProgress()
  // 改变样式
  document.body.style.setProperty("--el-color-primary", "#212121");
  document.body.style.setProperty("--el-border-color-base", "#212121");
  document.body.style.setProperty("--el-border-color-light", "#212121");
  document.body.style.setProperty("--el-color-black", "#212121");
  document.body.style.setProperty("--el-border-color-extra-light", "#9a9a9a");
  document.body.style.setProperty("--el-text-color-regular", "#212121");
})

async function onListenLog() {
  await listen("log", (e) => {
    let date = new Date()
    log.value += ('【' + date.toLocaleString() + '】' + e.payload + '\n')
  })
}

async function onListenProgress() {
  await listen("progress", (e) => {
    progress.value = parseFloat(e.payload) * 100;
    //progress.value = e.payload;
  })
}


function testConnect() {
  let db_conf = configStore.value.config;
  // 进入连接中状态，按钮等待
  connecting.value = true

  invoke('test_connect', {
    backend_end_point: db_conf.common.backend_endpoint,
    db_end_point: db_conf.common.db_endpoint,
    database: db_conf.common.db_database,
    user_name: db_conf.common.db_user,
    password: db_conf.common.db_password
  }).then((response) => {
    connecting.value = false
    console.log(response)
    if (response === "DbError") {
      infoBarShow.value = true;
      infoBarText.value = "数据库连接失败!";
    } else if (response === "BackendError") {
      infoBarShow.value = true;
      infoBarText.value = "后端服务连接失败!";
    } else if (response === "Success") {
      infoBarShow.value = true;
      infoBarText.value = "连接成功!";
    }
  })
}


</script>

<template>
  <v-card>
    <v-card-title class="text-lg-h5"><strong>基础配置</strong></v-card-title>
    <v-responsive class="mx-auto" max-width="2554">
      <v-text-field
          density="compact"
          variant="outlined"
          v-model="configStore.config.common.backend_endpoint"
          class="common_text_entry"
          hide-details="auto"
          label="后端地址" placeholder="192.168.2.2:8080"></v-text-field>
      <v-text-field
          density="compact"
          variant="outlined" v-model="configStore.config.common.db_endpoint" class="common_text_entry"
          hide-details="auto"
          label="数据库地址" placeholder="192.168.2.2:8600"></v-text-field>
      <v-text-field
          density="compact"
          variant="outlined" v-model="configStore.config.common.db_database" class="common_text_entry"
          hide-details="auto"
          label="数据库名称"></v-text-field>
      <v-text-field
          density="compact"
          variant="outlined" v-model="configStore.config.common.db_user"
          class="common_text_entry"
          hide-details="auto"
          label="数据库用户名"></v-text-field>
      <v-text-field
          density="compact"
          variant="outlined"
          v-model="configStore.config.common.db_password"
          class="common_text_entry"
          hide-details="auto"
          label="数据库密码"></v-text-field>
    </v-responsive>


    <v-btn color="success" class="w-100" :loading="connecting" @click="testConnect()">
      <strong>连接测试</strong>
    </v-btn>
    <v-snackbar v-model="infoBarShow" multi-line>
      {{ infoBarText }}
      <template v-slot:actions>
        <v-btn
            color="red"
            variant="text"
            @click="infoBarShow = false">
          Close
        </v-btn>
      </template>
    </v-snackbar>

    <div style="padding-top: 10px">
      <v-divider class="border-opacity-75"></v-divider>
    </div>

    <v-card-title class="text-lg-h5">
      <strong>运行配置</strong>
    </v-card-title>
    <v-tabs v-model="runningMode">
      <v-tab value="current">实时上数</v-tab>
      <v-tab value="history">历史上数</v-tab>
    </v-tabs>
    <v-window v-model="runningMode">
      <v-window-item value="current">
        <v-text-field
            density="compact"
            hide-details
            variant="outlined"
            v-model="timeInterval"
            class="common_text_entry"
            label="时间间隔(ms)"></v-text-field>
      </v-window-item>

      <v-window-item value="history">
        <div style="padding-bottom:10px;padding-top: 10px">
          <el-date-picker
              style="width: 100%;"
              v-model="timeScope"
              type="datetimerange"
              range-separator="至"
              start-placeholder="开始时间"
              end-placeholder="结束时间"/>
        </div>
        <v-text-field
            density="compact"
            hide-details
            variant="outlined"
            v-model="time_step"
            class="common_text_entry"
            label="步长(ms)">
        </v-text-field>
        <div style="padding: 10px">
          <v-progress-linear
              v-show="running"
              height="13px"
              color="teal"
              v-model:model-value="progress"
              stream>
            <template v-slot:default="{ value }">
              <strong style="font-size: small">进度:{{ Math.ceil(value) }}%</strong>
            </template>
          </v-progress-linear>
        </div>

      </v-window-item>
    </v-window>

    <v-btn style="margin-right: 1%;text-align: center" :disabled=running @click="run()" width="49%"
           color="success">
      <strong>运行</strong>
    </v-btn>
    <v-btn style="margin-left: 1%" :disabled=!running @click="stop()" width="49%" color="warning">
      <strong>停止</strong>
    </v-btn>
    <v-textarea
        style="padding-top: 10px"
        density="compact"
        :dense="true"
        v-model="log"
        label="日志"
        variant="outlined"
        class="logTextarea">
    </v-textarea>
  </v-card>

</template>

<style scoped>
.common_text_entry {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}


/deep/ .v-textarea textarea {
  line-height: 1em;
  font-size: small;
  height: 200px;
}

/deep/  .el-date-table-cell {
  background-color: #bebebe;
}
/deep/ .el-date-editor  {
  --el-input-bg-color: #222222;
  --el-text-color-placeholder: white;
  color: white;
}

/deep/ .el-date-editor .el-range-input{
  color:white;
}


</style>
