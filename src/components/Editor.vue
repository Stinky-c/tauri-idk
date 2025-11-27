<template>
  <div class="min-h-full p-4 bg-base-200">
    <div class="flex gap-4">
      <aside class="w-80">
        <div class="card bg-base-100 shadow">
          <div class="card-body">
            <h2 class="card-title">Preview / State</h2>
            <pre> {{ $props.filename }}</pre>
          </div>
        </div>
      </aside>

      <div
        class="flex-1 bg-neutral rounded-lg overflow-hidden"
        ref="editorRoot"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { EditorState, StateEffect, StateField, Text } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { oneDark } from "@codemirror/theme-one-dark";
import { indentWithTab } from "@codemirror/commands";

import { basicSetup } from "codemirror";

const props = defineProps<{
  filename: string;
}>();

const emit = defineEmits<{
  "model:update": [];
}>();
const editorRoot = ref();

// State Effects
const toggleHelp = StateEffect.define<boolean>();

// State Fields
const helpPanelState = StateField.define<boolean>({
  create: () => false,
  update(value, tr) {
    for (let e of tr.effects) if (e.is(toggleHelp)) value = e.value;
    return value;
  },
});

// Keymaps
const helpKeymap = [
  {
    key: "F1",
    run(view: EditorView) {
      view.dispatch({
        effects: toggleHelp.of(!view.state.field(helpPanelState)),
      });
      return true;
    },
  },
];

// wiring effects
function createHelpPanel(view: EditorView) {
  let dom = document.createElement("div");
  dom.textContent = "F1: Toggle the help panel";
  dom.className = "cm-help-panel";
  return { top: true, dom };
}

let editorState = EditorState.create({
  doc: Text.empty,
  extensions: [
    basicSetup,
    oneDark,
    EditorView.editable.of(true),
    keymap.of([indentWithTab]),
  ],
});

let editorView: EditorView | null = null;

EditorView.updateListener.of((update) => {
  update.changes;
});

onMounted(() => {
  editorView = new EditorView({
    state: editorState,
    parent: editorRoot.value,
  });
});

onBeforeUnmount(() => {
  // TODO: save to tauri store
});

onUnmounted(() => {
  // TODO: Destroy editorstate cleanly
  if (editorView) {
    editorView.destroy();
  }
});
</script>
