<script setup>
const props = defineProps(['items', 'active']);
const emits = defineEmits(['update:active']);
</script>
<template>
    <div class="select">
        <div 
            @click="() => $emit('update:active', item)"
            :class="item == active ? 'item active' : 'item'" 
            v-for="item in items"
        >
            <span>
                {{ item }}
            </span>
        </div>
    </div>
</template>
<style lang="scss" scoped>
    .select {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(1px, 1fr));

        .item {
            cursor: pointer;
            &.active {
                color: var(--light);
                cursor:not-allowed;
            }
            background: var(--background-second);
            height: 50px;

            border: 2px solid #FFFFFF06;
            border-right: none;

            color: var(--text);

            &:first-child {
                border-radius: 8px 0px 0px 8px;
            }

            &:last-child {
                border-radius: 0px 8px 8px 0px;
                border-right: 2px solid #FFFFFF06;
            }

            display: flex;
            justify-content: center;
            align-items: center;
        }
    }
</style>