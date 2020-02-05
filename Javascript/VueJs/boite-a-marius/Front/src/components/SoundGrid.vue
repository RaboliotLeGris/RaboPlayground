<template>
    <div class="tableContainer">
        <table>
            <tr v-for="(rows, rowIndex) in items" :key="rowIndex">
                <td v-for="(quote, quoteIndex) in rows" :key="quoteIndex">
                    <SoundButton :quote="quote"></SoundButton>
                </td>
            </tr>
        </table>
    </div>
</template>

<script>
    import SoundButton from "./SoundButton";
    import SoundData from "../SoundData";

    export default {
        name: 'SoundGrid',
        components: {
            SoundButton
        },
        data() {
            const itemPerRaw = 3;
            let rawData = SoundData;

            let shapedData = [];
            let currentRow = [];

            for (let i = 0; i < rawData.length; i++) {
                if (i !== 0 && i % itemPerRaw === 0) {
                    shapedData.push(currentRow);
                    currentRow = [];
                }
                currentRow.push(rawData[i]);
            }
            if (currentRow.length > 0) {
                shapedData.push(currentRow);
            }

            return {
                items: shapedData
            }
        }
    }
</script>

<style>
    .tableContainer {
        text-align: center;
    }

    table {
        margin: 0 auto;
        width: 60%;
    }
    td {
        padding: 1%;
    }
</style>