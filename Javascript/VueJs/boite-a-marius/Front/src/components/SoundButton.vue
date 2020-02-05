<template>
    <button v-on:click="playQuote"> {{ quote }}</button>
</template>

<script>
    export default {
        name: 'SoundButton',
        props: {
            quote: String,
        },
        methods: {
            playQuote: function () {
                let filename = this.quote.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "").replace(/ /gi, '-');
                let audio = new Audio(require(`../resources/sound/${filename}.mp3`));
                audio.play();
            }
        }
    }
</script>

<style>
    button {
        width: 100%;
        box-shadow:inset 0px 1px 0px 0px #7bde50;
        background:linear-gradient(to bottom, #55c720 5%, #56c420 100%);
        background-color:#55c720;
        border-radius:6px;
        border:1px solid #3b6e22;
        display:inline-block;
        cursor:pointer;
        color:#ffffff;
        font-family:Arial;
        font-size:15px;
        font-weight:bold;
        padding:24px 56px;
        text-decoration:none;
    }
</style>