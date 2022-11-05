<script setup lang="ts">
    import { preview } from "./impl";
</script>
<template>
    <main>
        <textarea id="list" style="flex: 20%"/>
            <div style="flex:10%">
                <button>Load</button>
                <button @click="preview">Preview</button>
            </div>
        <textarea id="inputbox" style="flex: 70%"/>
    </main>
</template>

<style scoped>
    main {
        display: flex;
        height: 35vh;
    }
</style>