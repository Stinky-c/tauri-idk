<template>
  <!-- component fills parent; editor root is set to stretch -->
  <div class="h-full flex flex-col">
    <div ref="editorRoot" class="flex-1 min-h-0 h-full" />
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import { EditorState } from "@codemirror/state";
import {
  EditorView,
  keymap,
  highlightActiveLine,
  lineNumbers,
  highlightActiveLineGutter,
} from "@codemirror/view";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { javascript } from "@codemirror/lang-javascript";
import { oneDark } from "@codemirror/theme-one-dark";
import { autocompletion } from "@codemirror/autocomplete";

const props = defineProps({
  modelValue: { type: String, default: "" },
  readOnly: { type: Boolean, default: false },
  extensions: { type: Array, default: () => [] },
});
const emit = defineEmits(["update:modelValue"]);

const editorRoot = ref(null);
let view = null;
let internalUpdate = false;

onMounted(() => {
  const startState = EditorState.create({
    doc: props.modelValue ?? "",
    extensions: [
      lineNumbers(),
      highlightActiveLineGutter(),
      highlightActiveLine(),
      keymap.of([...defaultKeymap, ...historyKeymap]),
      history(),
      autocompletion(),
      javascript(),
      oneDark,
      EditorView.editable.of(!props.readOnly),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          if (!internalUpdate) {
            const value = view.state.doc.toString();
            emit("update:modelValue", value);
          }
        }
      }),
      ...props.extensions,
    ],
  });

  view = new EditorView({
    state: startState,
    parent: editorRoot.value,
  });
});

onBeforeUnmount(() => {
  if (view) {
    view.destroy();
    view = null;
  }
});

watch(
  () => props.modelValue,
  (newVal) => {
    if (!view) return;
    const current = view.state.doc.toString();
    if (newVal !== current) {
      internalUpdate = true;
      const transaction = view.state.update({
        changes: { from: 0, to: view.state.doc.length, insert: newVal ?? "" },
      });
      view.dispatch(transaction);
      requestAnimationFrame(() => {
        internalUpdate = false;
      });
    }
  }
);
</script>

<!-- styles removed: Tailwind handles layout -->
