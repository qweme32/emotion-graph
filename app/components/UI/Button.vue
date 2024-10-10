<script setup lang="ts">
defineProps(['label', 'theme']);
</script>
<template>
    <div :class="theme == 'secondary' ? 'button secondary' : 'button'">
        <span>{{ label }}</span>
    </div>
</template>
<style scoped lang="scss">
    .button {
        background: var(--primary);

        &.secondary {
            background: var(--secondary);
        }
        
        height: 50px;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 8px;
        color: var(--light);

        cursor: pointer;

        span {
            font-size: 18px;
            font-weight: 600;
        }
    }
</style>