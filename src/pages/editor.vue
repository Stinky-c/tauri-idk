<template>
  <!-- root must use h-full so App.vue's router-view (which is h-full) can be fully occupied -->
  <div class="min-h-full p-4 bg-base-200">
    <h1 class="text-2xl font-semibold mb-4">CodeMirror editor (Vue)</h1>

    <!-- Use flex and allow the editor pane to shrink properly with min-h-0 on the flex container -->
    <div class="flex gap-4">
      <!-- subtract header height to let content fill remaining space -->
      <!-- editor pane: min-h-0 + flex-1 ensures CodeMirror can stretch inside a flex container -->
      <div class="flex-1 bg-neutral rounded-lg overflow-hidden">
        <CodeMirrorEditor
          v-model="code"
          :extensions="customExtensions"
          :read-only="false"
          class="h-full"
        />
      </div>

      <!-- right-side controls using DaisyUI card -->
      <aside class="w-80">
        <div class="card bg-base-100 shadow">
          <div class="card-body">
            <h2 class="card-title">Preview / State</h2>

            <label class="label">
              <span class="label-text">Filename</span>
            </label>
            <input
              v-model="filename"
              class="input input-bordered w-full mb-3"
            />

            <h3 class="text-sm font-medium">Current content</h3>
            <pre
              class="mt-2 p-2 bg-white rounded max-h-48 overflow-auto text-sm"
              >{{ code }}</pre
            >

            <div class="card-actions mt-3">
              <button class="btn btn-primary btn-sm" @click="loadExample">
                Load example
              </button>
              <button class="btn btn-ghost btn-sm" @click="clear">Clear</button>
            </div>
          </div>
        </div>
      </aside>
    </div>
  </div>
</template>

<script setup>
import CodeMirrorEditor from "@/components/Editor.vue";

const customExtensions = [];

const code = ref(`function hello(name) {
  console.log('Hello, ' + name)
}

hello('World')`);

const filename = ref("example.js");

function loadExample() {
  code.value = `// Example: Fibonacci (recursive)
function fib(n) {
  if (n < 2) return n
  return fib(n - 1) + fib(n - 2)
}

console.log(fib(10))`;
}

function clear() {
  code.value = "";
}
</script>
