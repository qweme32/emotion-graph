<script setup lang="ts">
defineProps(["placeholder", "type", "modelValue"]);
defineEmits(["update:modelValue"]);
</script>
<template>
    <input 
        class="ui-input" 
        :placeholder="placeholder" 
        :type="type ? type : 'text'"
        :value="modelValue" @input="$emit('update:modelValue', $event.target.value)"
    >
</template>
<style lang="scss" scoped>
    .ui-input {
        background: var(--secondary);
        width: calc(100% - 36px);
        padding-left: 18px;
        padding-right: 18px;
        padding-top: 12px;
        padding-bottom: 12px;
        outline: none;
        border: none;
        border-radius: 8px;
        font-size: 20px;
        font-weight: 500;
        color: var(--light);

        &::placeholder {
            color: var(--text);
        }
    }
</style>