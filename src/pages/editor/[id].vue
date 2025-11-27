<script setup lang="ts">
import Editor from "@/components/Editor.vue";
</script>

<template>
  <Editor :filename="$route.params.id" />
</template>
