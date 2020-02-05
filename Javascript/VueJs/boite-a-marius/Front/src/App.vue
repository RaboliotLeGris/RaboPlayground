<template>
    <div id="app">
        <SoundBoardHeader></SoundBoardHeader>
        <SoundGrid></SoundGrid>
    </div>
</template>

<script>
    import SoundBoardHeader from "./components/SoundBoardHeader";
    import SoundGrid from "./components/SoundGrid";

    export default {
        name: 'app',
        components: {
            SoundBoardHeader,
            SoundGrid
        }
    }
</script>

<style>
    #app {
        font-family: 'Avenir', Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
        color: #2c3e50;
        margin-top: 60px;
    }
</style>
