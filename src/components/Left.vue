<script setup>
import {useConfigStore} from '../store/index.js'
import {ref, reactive, onMounted} from 'vue'
import {invoke} from "@tauri-apps/api";
import {listen} from '@tauri-apps/api/event'

// 样式控制
const dialog = ref(false)
const infoBarShow = ref(false);
const infoBarText = ref("");
const dialogMod = ref("")
const modelState = reactive(new Map)

// 核心数据
const configStore = useConfigStore()
const modelDialog = reactive({
  modelList: [],
  objectList: [],
  selectModel: null,
  selectObject: [],
  propertyConfs: []
})

onMounted(() => {
  onListenModelState()
})

async function onListenModelState() {
  await listen("model_state", (e) => {
    modelState.set(e.payload.model_name, e.payload.state)
    console.log(modelState)
  })
}

function getModelList() {
  return invoke('get_model_list').then((response) => {
    return response
  }).catch((error) => {
        console.log(error)
        infoBarShow.value = true;
        infoBarText.value = error;
      }
  )
}

function getObjectList(modelName) {
  return invoke('get_object_list_by_model_name', {model_name: modelName}).then((response) => {
    return response
  }).catch((error) => {
        infoBarShow.value = true;
        infoBarText.value = error;
      }
  )
}

function getModelPropertyDef(modelName) {
  return invoke('get_model_property_def', {model_name: modelName}).then((response) => {
    return response
  }).catch((error) => {
        infoBarShow.value = true;
        infoBarText.value = error;
        return null
      }
  )
}

async function onModelSelected() {
  // 获取实例列表
  modelDialog.objectList = await getObjectList(modelDialog.selectModel)
  modelDialog.selectObject = []
  // 渲染属性表格
  modelDialog.propertyConfs = await getModelPropertyDef(modelDialog.selectModel)
}

async function openModelConfigDialog(mod, item) {
  if (mod === "add") {
    dialogMod.value = 'add'
    modelDialog.modelList = await getModelList()
    if (modelDialog.modelList !== undefined) {
      dialog.value = true
    }
  }
  if (mod === "update") {
    dialogMod.value = 'update'
    modelDialog.modelList = await getModelList()
    modelDialog.selectModel = item.modelName
    await onModelSelected()
    modelDialog.selectObject = item.objectNameList

    // 填充值
    let sourcePropertyConfs = item.propertyConfs
    sourcePropertyConfs.forEach(function (sourcePropertyDef, index) {
      let targetPropertyDef = modelDialog.propertyConfs.find(function (propertyDef) {
        return propertyDef.name === sourcePropertyDef.name
      })

      targetPropertyDef.enable = sourcePropertyDef.enable;
      targetPropertyDef.const = sourcePropertyDef.const;
      targetPropertyDef.is_random = sourcePropertyDef.is_random;
      targetPropertyDef.lower_bound = sourcePropertyDef.lower_bound;
      targetPropertyDef.upper_bound = sourcePropertyDef.upper_bound;
      targetPropertyDef.dp = sourcePropertyDef.dp;
    })
    dialog.value = true
  }
}

async function removeModelConf(item) {
  let index = configStore.config.modelConfs.indexOf(item)
  configStore.config.modelConfs.splice(index, 1)
}

function addCurrentModelConfig(index) {
  // TODO 若存在该模型配置，则更新，否则新建
  dialog.value = false;


  // 1.构建 modelConfs_item
  // 1.1 处理 property def->去除未选择
  modelDialog.propertyConfs = modelDialog.propertyConfs.filter(function (propertyDef) {
    return propertyDef.enable !== undefined && propertyDef.enable === true
  })

  // 1.2 处理 property def->类型转换
  modelDialog.propertyConfs.forEach(function (propertyDef, index) {
    if (propertyDef.const !== undefined) {
      if (propertyDef.data_type === "double") {
        propertyDef.const = parseFloat(propertyDef.const)
      }
      if (propertyDef.data_type === "long") {
        propertyDef.const = parseInt(propertyDef.const)
      }
      if (propertyDef.data_type === "boolean") {
        propertyDef.const = Boolean(propertyDef.const)
      }
    }
    if (propertyDef.lower_bound !== undefined) {
      propertyDef.lower_bound = parseFloat(propertyDef.lower_bound)
    }
    if (propertyDef.upper_bound !== undefined) {
      propertyDef.upper_bound = parseFloat(propertyDef.upper_bound)
    }
    if (propertyDef.dp !== undefined) {
      propertyDef.dp = parseFloat(propertyDef.dp)
    }
  });

  let model_conf_item = {}
  model_conf_item.modelName = modelDialog.selectModel

  const display_name = modelDialog.modelList.find(function (m) {
    return m.name === modelDialog.selectModel;
  }).display_name;
  model_conf_item.modelDisplayName = display_name
  model_conf_item.objectNameList = modelDialog.selectObject
  model_conf_item.propertyConfs = modelDialog.propertyConfs

  //console.log(JSON.stringify(model_conf_item))
  if (index === undefined) {
    configStore.config.modelConfs.push(model_conf_item)
  } else {
    configStore.config.modelConfs.splice(index, 0, model_conf_item)
  }

  // 清空 dialog
  clearModelDialog()
}

function clearModelDialog() {
  dialog.value = false
  // 清空 dialog
  modelDialog.modelList = []
  modelDialog.objectList = []
  modelDialog.propertyConfs = []
  modelDialog.selectModel = null
  modelDialog.selectObject = []
}


function updateCurrentModelConfig() {
  // 找到当前 config 的 index，找到 对应下标的 pinia 配置，新增配置
  let targetModelConf = configStore.config.modelConfs.find(function (modelConf) {
    return modelConf.modelName === modelDialog.selectModel
  })
  let index = configStore.config.modelConfs.indexOf(targetModelConf)

  configStore.config.modelConfs.splice(index, 1)

  addCurrentModelConfig(index)
}
</script>

<template>
  <v-card>
    <!--  info bar-->
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
    <!--  添加模型-->
    <v-btn style="margin-left: 20px" @click="openModelConfigDialog('add',null)" color="primary">
      添加模型配置
      <v-icon end icon="mdi-plus"></v-icon>
    </v-btn>

    <v-dialog persistent v-model="dialog" style="width: 80%;margin: 0 auto">
      <v-card>
        <v-card-text>
          <!--模型-->
          <v-autocomplete
              style="padding: 10px"
              density="compact"
              variant="outlined"
              v-model="modelDialog.selectModel"
              :items="modelDialog.modelList"
              item-title="display_name"
              item-value="name"
              label="模型"
              @update:modelValue="onModelSelected()">

            <template v-slot:item="{ props, item }">
              <v-list-item
                  v-bind="props"
                  :title="item?.raw?.display_name"
                  :subtitle="item?.raw?.name">
              </v-list-item>
            </template>
          </v-autocomplete>

          <!--实例-->
          <v-autocomplete
              style="padding: 10px"
              density="compact"
              variant="outlined"
              v-model="modelDialog.selectObject"
              :items="modelDialog.objectList"
              item-title="display_name"
              item-value="name"
              chips
              closable-chips
              multiple
              label="实例">

            <template v-slot:item="{ props, item }">
              <v-list-item
                  v-bind="props"
                  :title="item?.raw?.display_name"
                  :subtitle="item?.raw?.name">
              </v-list-item>
            </template>
          </v-autocomplete>

          <!--属性配置-->
          <v-table density="compact" fixed-header>
            <thead>
            <tr>
              <th>启用</th>
              <th>属性名称</th>
              <th>属性标识</th>
              <th>数据类型</th>
              <th>固定值</th>
              <th>随机值</th>
              <th>下界</th>
              <th>上界</th>
              <th>小数点位数</th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="item in modelDialog.propertyConfs" :key="item.name">
              <td>
                <v-checkbox
                    density="compact"
                    v-model="item.enable"></v-checkbox>
              </td>
              <td>{{ item.display_name }}</td>
              <td>{{ item.name }}</td>
              <td>{{ item.data_type }}</td>
              <td>
                <v-text-field
                    density="compact"
                    variant="outlined"
                    v-show="!item.is_random&&item.enable"
                    v-model="item.const"
                    label="固定值"></v-text-field>
              </td>
              <td>
                <v-checkbox
                    density="compact"
                    v-show="item.enable"
                    v-model="item.is_random"></v-checkbox>
              </td>
              <td>
                <v-text-field
                    density="compact"
                    variant="outlined"
                    v-show="item.is_random&&item.enable&&(item.data_type==='double'||item.data_type==='long')"
                    v-model="item.lower_bound"
                    label="下界"></v-text-field>
              </td>
              <td>
                <v-text-field
                    density="compact"
                    variant="outlined"
                    v-show="item.is_random&&item.enable&&(item.data_type==='double'||item.data_type==='long')"
                    v-model="item.upper_bound"
                    label="上界"></v-text-field>
              </td>
              <td>
                <v-text-field
                    density="compact"
                    variant="outlined"
                    v-show="item.is_random&&item.data_type==='double'&&item.enable"
                    v-model="item.dp"
                    label="小数点位数"></v-text-field>
              </td>
            </tr>
            </tbody>
          </v-table>
        </v-card-text>
        <v-card-actions style="margin: 0 auto">
          <v-btn color="primary" v-show="dialogMod==='add'" @click="addCurrentModelConfig()">确认</v-btn>
          <v-btn color="primary" v-show="dialogMod==='update'" @click="updateCurrentModelConfig()">更新</v-btn>
          <v-btn color="primary" @click="clearModelDialog">取消</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>


    <!--模型配置列表-->
    <v-table density="compact" fixed-header height="500px">
      <tbody>
      <tr v-for="item in configStore.config.modelConfs" :key="item.name">
        <td>
          <v-card color="#ecf2fe" style="padding:0px 0;margin: 15px 0;">
            <v-card-title class="text-h6">
              <!--异常时-->
              <v-badge v-show="modelState.get(item.modelName)!=null&&!modelState.get(item.modelName)" content="异常"
                       color="error">
                <div>
                  <strong>{{ item.modelDisplayName }}</strong>
                  <v-icon icon="mdi-bell-ring"></v-icon>
                </div>
              </v-badge>
              <!--正常时-->
              <v-badge v-show="modelState.get(item.modelName)" content="正常" color="success">
                <div>
                  <strong>{{ item.modelDisplayName}}   </strong>
                  <v-icon icon="mdi-bell-ring"></v-icon>
                </div>
              </v-badge>
              <!--等待中-->
              <div v-show="modelState.get(item.modelName)==null">
                <strong>{{ item.modelDisplayName }}</strong>
                <v-icon icon="mdi-bell-ring"></v-icon>
              </div>

            </v-card-title>

            <v-card-subtitle>{{ item.modelName }}</v-card-subtitle>

            <div style="padding: 10px">
              <v-btn @click="openModelConfigDialog('update',item)"
                     color="primary"
                     variant="flat">修改
                <v-icon end icon="mdi-pen"></v-icon>
              </v-btn>
              <v-btn style="margin-left: 10px"
                     @click="removeModelConf(item)"
                     color="error">删除
                <v-icon end icon="mdi-delete-outline"></v-icon>
              </v-btn>
            </div>
          </v-card>

        </td>
      </tr>
      </tbody>
    </v-table>
  </v-card>

</template>

<style scoped>
/deep/ .v-text-field input {
  font-size: 0.8em;
}

/deep/ .v-input__details {
  display: none;
}


</style>
