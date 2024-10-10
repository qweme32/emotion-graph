<script lang="js" setup>
const data = ref([]);

const period = useState("period", () => "Today");
const rate = useState("rate", () => "0");
const api = useApi();
const router = useRouter();
const username = useState("username", () => "");

api.check().then(async value => {
    if (!value) {
        router.push('/auth');
    }

    let items = await api.reports(value, 1);

    for (const item of items) {
        data.value.push({
            date: item.date,
            rate: item.rate,
        });
    }

    username.value = value;
})

const uploadRate = async (rate) => {
    if (!await api.report(parseInt(rate))) {
        api.token.value = "";
        router.push('/auth');
    };

    data.value.splice(0, 0, {
        date: new Date().toISOString(),
        rate: parseInt(rate),
    });
};

watch(period, async () => {
    let periodInDays = 1;
    if (period.value == "Week") {
        periodInDays = 7;
    } else if (period.value == "Month") {
        periodInDays = 30;
    }

    let items = await api.reports(username.value, periodInDays);

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

        <UISelect v-model:active="rate" :items="['-3', '-2', '-1', '0', '1', '2', '3']" />

        <UIButton @click="() => uploadRate(rate)" label="Submit" />

        <UIButton theme="secondary" @click="() => $router.push(`/shared/${username}`)" label="Share" />
    </Layout>
</template>
