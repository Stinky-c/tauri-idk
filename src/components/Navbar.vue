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
      <RouterLink to="/about" class="link link-hover link-success"
        >About</RouterLink
      >
    </div>
  </nav>
</template>
