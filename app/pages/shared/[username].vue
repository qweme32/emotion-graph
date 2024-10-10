<script lang="js" setup>
const data = ref([]);

const period = useState("period", () => "Today");
const rate = useState("rate", () => "0");
const api = useApi();
const router = useRouter();
const route = useRoute();

let init = async () => {
    let items = await api.reports(route.params.username, 1);

    if (!items) {
        return router.push('/me');
    }

    for (const item of items) {
        data.value.push({
            date: item.date,
            rate: item.rate,
        });
    }
}

init();

watch(period, async () => {
    let periodInDays = 1;
    if (period.value == "Week") {
        periodInDays = 7;
    } else if (period.value == "Month") {
        periodInDays = 30;
    }

    let items = await api.reports(route.params.username, periodInDays);

    for (const _ in data.value) { data.value.pop(); };

    for (const item of items) {
        data.value.push({
            date: item.date,
            rate: item.rate,
        });
    }
})
</script>
<template>
    <Layout>
        <Brand />

        <UISelect v-model:active="period" :items="['Month', 'Today', 'Week']" />

        <Chart :data="data" />

        <div class="text">this graph was shared with you by <span>{{ $route.params.username }}</span></div>

        <UIButton theme="secondary" @click="() => $router.push(`/me`)" label="Back to my graph" />
    </Layout>
</template>
<style scoped lang="scss">
.text {
    text-align: center;
    color: var(--text);
    font-weight: 400;
    font-size: 12px;

    span {
        text-decoration: underline;
    }
}
</style>
