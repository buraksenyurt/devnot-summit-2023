<template>
    <el-form :model="workItemForm" size="default">
        <el-form-item label="Title" prop="title">
            <el-input v-model="workItemForm.title" placeholder="Görev başlığı"></el-input>
        </el-form-item>
        <el-form-item label="Summary" prop="summary">
            <el-input v-model="workItemForm.summary" placeholder="Görev bilgisi" type="textarea"></el-input>
        </el-form-item>
        <el-form-item>
            <el-text>Business Value</el-text>
            <el-input-number min="0" max="13" step="1" v-model="workItemForm.business_value"/>
        </el-form-item>
        <el-from-item>
            <el-button type="primary" @click="submitItem">Kaydet</el-button>
        </el-from-item>
    </el-form>
</template>

<script lang="ts">
export const componentName = 'WorkItemForm'
export default {
    name: 'WorkItemForm',
    inheritAttrs: false,
    customOptions: {}
}
</script>

<script setup lang="ts">
import { reactive, defineProps } from 'vue'
import { ElMessage } from 'element-plus'

const props = defineProps(['submitItem'])
const workItemForm = reactive({
    title: '',
    summary: '',
    business_value: 0,
});

const submitItem = () => {
    if (workItemForm.title.length < 10) {
        ElMessage({
            message: 'Görev başlığı en az 10 karakter olmalıdır.',
            type: 'error'
        })
        return
    }

    if (workItemForm.summary.length < 20) {
        ElMessage({
            message: 'Görev hakkında bir açıklama girilmeli. Min 20 karakter.',
            type: 'error'
        })
        return
    }

    props.submitItem({
        title: workItemForm.title,
        summary: workItemForm.summary,
        business_value: workItemForm.business_value
    });
    workItemForm.title = '';
    workItemForm.summary = '';
    workItemForm.business_value = 0;
}

</script>