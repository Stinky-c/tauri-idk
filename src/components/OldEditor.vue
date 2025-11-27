<template>
  <div ref="editorRoot" class="flex-1 min-h-0 h-full" />
</template>

<script setup lang="ts">
import { EditorState, Extension } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { javascript } from "@codemirror/lang-javascript";
import { oneDark } from "@codemirror/theme-one-dark";

import { basicSetup } from "codemirror";

const props = defineProps<{
  extensions: Extension[];
  readOnly: boolean;
  modelValue: string;
}>();
const emit = defineEmits(["update:modelValue"]);

const editorRoot = ref(null);
let view: Ref<EditorView | null> = ref(null);
let internalUpdate = false;

onMounted(() => {
  const startState = EditorState.create({
    extensions: [
      basicSetup,
      javascript(),
      oneDark,
      EditorView.editable.of(!props.readOnly),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          if (!internalUpdate) {
            const value = view!.state.doc.toString();
            emit("update:modelValue", value);
          }
        }
      }),
      ...props.extensions,
    ],
  });

  view.value = new EditorView({
    state: startState,
    parent: editorRoot.value,
  });
});

onBeforeUnmount(() => {
  if (view) {
    view.destroy();
    view.value = null;
  }
});

watch(
  () => props.modelValue,
  (newVal) => {
    if (!view) return;
    const current = view.value!.state.doc.toString();
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
