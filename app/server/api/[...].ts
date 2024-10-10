import { joinURL } from "ufo";

export default defineEventHandler(async event => {
    const proxy = useRuntimeConfig().apiUrl;

    const path = event.path.replace("/api", "");
    const target = joinURL(proxy, path);

    return proxyRequest(event, target);
})