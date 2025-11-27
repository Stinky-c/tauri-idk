<script setup lang="ts">
import { Ref, ref } from "vue";

const pages: Ref<{ path: string; label: string }[]> = ref([
  {
    path: "/about",
    label: "About",
  },
  {
    path: "/editor",
    label: "Editor",
  },
  {
    path: "/test",
    label: "Test",
  },
]);
</script>
<template>
  <nav class="navbar bg-base-100 shadow-sm">
    <div class="navbar-start">
      <RouterLink to="/" class="btn btn-ghost text-xl">Home</RouterLink>
    </div>
    <div class="navbar-center">
      <p>
        Current Path: <kbd class="kbd kbd-sm">{{ $route.fullPath }}</kbd>
      </p>
    </div>
    <div class="navbar-end">
      <RouterLink to="/about" class="link link-hover">About</RouterLink>
      <RouterLink to="/editor" class="link link-hover">Editor</RouterLink>
      <RouterLink to="/Test" class="link link-hover">Test</RouterLink>
    </div>
  </nav>
</template>
