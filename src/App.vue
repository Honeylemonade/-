<script setup>
import Right from "./components/Right.vue"
import Left from "./components/Left.vue"
import Header from "./components/Header.vue"
</script>

<template>
  <v-layout>
    <v-app-bar variant="outlined">
      <Header/>
    </v-app-bar>
    <v-main>
      <v-container fluid>
        <v-row dense>
          <v-col cols="6">
            <Left style="padding: 3%; margin: 1%;" class="h-100"/>
          </v-col>
          <v-col cols="6">
            <Right style="padding: 3%; margin: 1%;" class="h-100"/>
          </v-col>
        </v-row>
      </v-container>
    </v-main>

  </v-layout>
</template>

<style scoped>
</style>
