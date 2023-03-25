<script setup>
import Right from "./components/Right.vue"
import Left from "./components/Left.vue"
import Header from "./components/Header.vue"
</script>

<template>
  <v-layout>
    <Header />
    <v-main>
      <Left />
    </v-main>
    <v-main>
      <Right />
    </v-main>
  </v-layout>
</template>

<style scoped></style>
