<template>
  <div id="app">
    <Titlebar />
    <router-view />
    <el-backtop :right="10" :bottom="10" />
  </div>
</template>

<script setup>
import Titlebar from './components/Titlebar.vue'

</script>