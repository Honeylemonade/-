<script setup>
import { useConfigStore } from '../store/index.js'

const configStore = useConfigStore()
var desserts = [
    {
        name: 'Frozen Yogurt',
        calories: 159,
    },
    {
        name: 'Ice cream sandwich',
        calories: 237,
    },
    {
        name: 'Eclair',
        calories: 262,
    }
]
</script>

<template>
    <v-card style="padding: 5%; margin: 5%;">
        <v-table fixed-header height="500px">
            <thead>
                <tr>
                    <th class="text-left">
                        Name
                    </th>
                    <th class="text-left">
                        Calories
                    </th>
                    <th class="操作">
                        Calories
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="item in desserts" :key="item.name">
                    <td>{{ item.name }}</td>
                    <td>{{ item.calories }}</td>
                    <td><v-btn @click=click(item) size="x-small" icon="mdi-heart" color="primary"></v-btn></td>

                </tr>
            </tbody>
        </v-table>
    </v-card>
</template>

<style scoped></style>
