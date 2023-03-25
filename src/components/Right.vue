<script setup>
import { useConfigStore } from '../store/index.js'
import { ref } from 'vue'
import Service from "../service.vue"

var switchMe = false
const configStore = useConfigStore()
const service = ref(null)

function testConnect() {
  service.value.testDBConnect()
}
</script>

<template>
  <Service ref="service"></Service>
  <v-card style="padding: 5%; margin: 5%;">
    <v-card-title class="text-h6 text-md-h5 text-lg-h4">基础配置</v-card-title>
    <v-responsive class="mx-auto" max-width="2554">
      <v-text-field v-model="configStore.config.common.backend_endpoint" class="common_text_entry" hide-details="auto"
        label="后端地址" placeholder="192.168.2.2:8080"></v-text-field>
      <v-text-field v-model="configStore.config.common.db_endpoint" class="common_text_entry" hide-details="auto"
        label="数据库地址" placeholder="192.168.2.2:8600"></v-text-field>
      <v-text-field v-model="configStore.config.common.db_database" class="common_text_entry" hide-details="auto"
        label="数据库名称"></v-text-field>
      <v-text-field v-model="configStore.config.common.db_user" class="common_text_entry" hide-details="auto"
        label="数据库用户名"></v-text-field>
      <v-text-field v-model="configStore.config.common.db_password" class="common_text_entry" hide-details="auto"
        label="数据库密码"></v-text-field>

    </v-responsive>
    <v-btn @click="testConnect">连接测试</v-btn>

    <v-card-title class="text-h6 text-md-h5 text-lg-h4">运行配置</v-card-title>
    <v-radio-group>
      <v-radio label="实时上数" value="running"></v-radio>
      <v-radio label="历史上数" value="history"></v-radio>
    </v-radio-group>
    <v-switch v-model="switchMe" label="success" color="success" value="success" hide-details>
      <template v-slot:label>
        运行状态:
        <v-progress-circular :indeterminate="switchMe" size="24" class="ms-2"></v-progress-circular>
      </template></v-switch>
  </v-card>
</template>

<style scoped>
.common_text_entry {
  margin: 0 auto;
  padding-bottom: 1rem;
  font-weight: bold;
}
</style>
