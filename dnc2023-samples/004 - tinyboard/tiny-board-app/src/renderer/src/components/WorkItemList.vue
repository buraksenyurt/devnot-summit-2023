<template>
  <el-row>
    <el-col :span="18" :offset="0" style="width:100%">
      <h1>Görevler</h1>
      <WorkItemForm :submitItem="createItem">
        
      </WorkItemForm>
      <el-table :data="workItems">
        <el-table-column prop="title" label="Title" width="150" />
        <el-table-column prop="summary" label="Summary" width="200" />
        <el-table-column prop="business_value" label="Value" width="200">
          <template #default="scope">
            <el-space wrap>
              <el-input-number min="0" max="13" step="1" v-model="scope.row.business_value"
                @click="updateItem(scope.row)" />
            </el-space>
          </template>
        </el-table-column>
        <el-table-column fixed="right" label="Operations" width="100">
          <template #default="scope">
            <el-space wrap>
              <el-switch v-model="scope.row.completed" @click="updateItem(scope.row)" />
              <el-popconfirm confirm-button-text="Eminim" cancel-button-text="Vezgeçtim" icon="el-icon-info"
                title="Görevi silmek istediğin emin misin?" @confirm="deleteItem(scope.row)" @cancel="cancelDelete">
                <template #reference>
                  <el-button size="default" type="danger">Görevi Sil</el-button>
                </template>
              </el-popconfirm>
            </el-space>
          </template>
        </el-table-column>
      </el-table>
    </el-col>
  </el-row>
</template>

<script lang="ts">
export default {
  name: 'WorkItemList',
  inheritAttrs: false,
  customOptions: {}
}
</script>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import WorkItemForm from './WorkItemForm.vue'
import axios from 'axios'

interface WorkItem {
  id?: number | string
  title: string
  summary: string
  business_value: number
  completed: boolean
}

const workItems = ref([])

onMounted(async () => {
  await loadWorkItems()
})

const loadWorkItems = async () => {
  console.log("Servis çağrısı yapılacak")
  const response = await axios.get('http://127.0.0.1:7000/workitems')
  console.log(response)
  workItems.value = response.data
}

const createItem = async (payload: WorkItem) => {
  await axios.post('http://127.0.0.1:7000/workitems', {
    title: payload.title,
    summary: payload.summary,
    business_value: payload.business_value,
    completed: false
  })
  ElMessage({
    message: 'WorkItem oluşturuldu',
    type: 'success'
  })
  await loadWorkItems()
}

const updateItem = async (payload: WorkItem) => {
  console.log(payload)
  await axios.put(`http://127.0.0.1:7000/workitems/${payload.id}`, {
    title: payload.title,
    summary: payload.summary,
    business_value: payload.business_value,
    completed: payload.completed
  })

  ElMessage({
    message: 'WorkItem güncellendi',
    type: 'success'
  })
  await loadWorkItems()
}

const deleteItem = async (payload: WorkItem) => {
  await axios.delete(`http://127.0.0.1:7000/workitems/${payload.id}`)

  ElMessage({
    message: 'WorkItem silindi',
    type: 'success'
  })
  await loadWorkItems()
}

const cancelDelete = () => {
  ElMessage({
    message: 'Silme işlemi iptal edildi',
    type: 'info'
  })
}
</script>
