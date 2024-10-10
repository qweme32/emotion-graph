<template>
    <div>
        <div class="layout">
            <div class="content">
                <slot/>
            </div>
        </div>
        <div class="footer">Powered by Nuxt with Actix-Web<br>With <3 by fake</div>
    </div>
</template>

<style lang="scss" scoped>
    .footer {
        display: flex;
        height: 5svh;
        text-align: center;
        color: var(--text);
        font-size: 12px;
        
        justify-content: center;
        align-items: center;
    }
    .layout {
        display: flex;
        height: 95svh;
        justify-content: center;
        align-items: center;
        margin-left: 28px;
        margin-right: 28px;

        @media screen and (min-width: 1000px) {
            margin-right: 100px;
            margin-left: 100px;
        }

        @media screen and (min-width: 1200px) {
            margin-right: 200px;
            margin-left: 200px;
        }

        @media screen and (min-width: 1500px) {
            margin-right: 300px;
            margin-left: 300px;
        }

        @media screen and (min-width: 1700px) {
            margin-right: 500px;
            margin-left: 500px;
        }

        @media screen and (min-width: 2000px) {
            margin-right: 700px;
            margin-left: 700px;
        }



        .content {
            width: 100%;
            display: grid;
            grid-gap: 12px;
        }
    }
</style>