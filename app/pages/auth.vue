<script setup lang="ts">
const username = useState("username", () => "");
const password = useState("password", () => "");

const api = useApi();
const router = useRouter();

api.check().then(value => {
    value ? router.push('/') : {};
});

async function authorize() {
    if (await api.authorize(username.value, password.value)) {
        router.push('/');
    } else {
        username.value = "";
        password.value = "";
    }
}
</script>
<template>
    <Layout>
        <Brand />
        <UIInput v-model="username" placeholder="root"/>
        <UIInput v-model="password" placeholder="********" type="password"/>
        <UIButton @click="authorize" label="Authorize"/>
    </Layout>
</template>