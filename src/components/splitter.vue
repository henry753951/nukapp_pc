<style lang="scss">
  .vue-splitter {
    display: grid;
    height: inherit;
    &.vertical {
      width: 100%;
      > .splitter {
        cursor: ew-resize;
        width: 5px;
      }
    }
    &.horizontal {
      height: 100%;
      > .splitter {
        cursor: ns-resize;
        height: 5px;
      }
    }
    .splitter {
      background-color: #9e9e9e;
    }
    .splitter-pane {
      overflow-y: auto;
    }
    .hide-pane {
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
      font-size: larger;
    }
  }
</style>
<template>
  <div
    :style="{ userSelect, gridTemplate }"
    class="vue-splitter"
    :class="{ horizontal: isHorizontal, vertical: !isHorizontal }"
    ref="containerRef">
    <div class="splitter-pane" :class="leftPaneClass">
      <slot
        name="left-pane"
        v-if="!isActive || !hideWhenDrag['left-pane']"></slot>
      <div class="hide-pane" v-else-if="!isHorizontal">
        {{ hideText["left-pane"] }}
      </div>
      <slot
        name="top-pane"
        v-if="!isActive || !hideWhenDrag['top-pane']"></slot>
      <div class="hide-pane" v-else-if="isHorizontal">
        {{ hideText["top-pane"] }}
      </div>
    </div>
    <div
      class="splitter"
      :class="{ active: isActive }"
      @mousedown="onSplitterMouseDown"
      @touchstart.passive="onSplitterTouchDown"
      @click="onSplitterClick" />
    <div class="splitter-pane" :class="rightPaneClass">
      <slot
        name="right-pane"
        v-if="!isActive || !hideWhenDrag['right-pane']"></slot>
      <div class="hide-pane" v-else-if="!isHorizontal">
        {{ hideText["right-pane"] }}
      </div>
      <slot
        name="bottom-pane"
        v-if="!isActive || !hideWhenDrag['bottom-pane']"></slot>
      <div class="hide-pane" v-else-if="isHorizontal">
        {{ hideText["bottom-pane"] }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
  import { computed, ref } from "vue";

  const props = withDefaults(
    defineProps<{
      isHorizontal?: boolean;
      percent?: number;
      initialPercent?: number | string;
      hideWhenDrag: {
        "left-pane"?: boolean;
        "right-pane"?: boolean;
        "top-pane"?: boolean;
        "bottom-pane"?: boolean;
      };
      hideText?: {
        "left-pane"?: string;
        "right-pane"?: string;
        "top-pane"?: string;
        "bottom-pane"?: string;
      };
    }>(),
    {
      isHorizontal: false,
      initialPercent: 50,
      hideWhenDrag: () => ({
        "left-pane": true,
        "right-pane": true,
        "top-pane": true,
        "bottom-pane": true,
      }),
      hideText: () => ({
        "left-pane": "Hide",
        "right-pane": "Hide",
        "top-pane": "Hide",
        "bottom-pane": "Hide",
      }),
    }
  );

  const emit = defineEmits<{
    (event: "update:percent", value: number): void;
    (event: "splitter-click"): void;
  }>();

  const isActive = ref(false);
  const percent = ref(50);
  const hasMoved = ref(false);
  const dragOffset = ref(0);

  const containerRef = ref<HTMLElement>();

  const modelPercent = computed<number>({
    get() {
      return props.percent ?? percent.value;
    },
    set(value) {
      emit("update:percent", value);
      percent.value = value;
    },
  });

  modelPercent.value = Number(props.initialPercent);

  const leftPaneClass = computed(() =>
    props.isHorizontal ? "top-pane" : "left-pane"
  );
  const rightPaneClass = computed(() =>
    props.isHorizontal ? "bottom-pane" : "right-pane"
  );
  const gridTemplate = computed(() =>
    props.isHorizontal
      ? `${modelPercent.value}% auto 1fr / none`
      : `none / ${modelPercent.value}% auto 1fr`
  );
  const userSelect = computed(() => (isActive.value ? "none" : "auto"));

  function onSplitterClick() {
    if (!hasMoved.value) {
      emit("splitter-click");
    }
  }

  function onSplitterMouseDown(e: MouseEvent) {
    dragOffset.value = props.isHorizontal ? e.offsetY : e.offsetX;
    onSplitterDown();
  }

  function onSplitterTouchDown() {
    dragOffset.value = 0;
    onSplitterDown();
  }

  function onSplitterDown() {
    isActive.value = true;
    hasMoved.value = false;
    addBodyListeners();
  }

  function addBodyListeners() {
    document.body.addEventListener("mousemove", onBodyMouseMove);
    document.body.addEventListener("touchmove", onBodyTouchMove);
    document.body.addEventListener("touchend", onBodyUp, { once: true });
    document.body.addEventListener("mouseup", onBodyUp, { once: true });
  }

  function onBodyTouchMove(e: TouchEvent) {
    if (isActive.value) {
      calculateSplitterPercent(e.touches[0]);
    }
  }

  function onBodyMouseMove(e: MouseEvent) {
    if (e.buttons && e.buttons === 0) {
      isActive.value = false;
      removeBodyListeners();
    }
    if (isActive.value) {
      calculateSplitterPercent(e);
    }
  }

  function calculateSplitterPercent(e: MouseEvent | Touch) {
    let offset = dragOffset.value;
    let target = containerRef.value as HTMLElement;
    let percent = 0;
    if (props.isHorizontal) {
      offset += target.offsetTop;
      while (target.offsetParent) {
        target = target.offsetParent as HTMLElement;
        offset += target.offsetTop;
      }
      percent =
        Math.floor(
          ((e.pageY - offset) / containerRef.value!.offsetHeight) * 10000
        ) / 100;
    } else {
      offset += target.offsetLeft;
      while (target.offsetParent) {
        target = target.offsetParent as HTMLElement;
        offset += target.offsetLeft;
      }
      percent =
        Math.floor(
          ((e.pageX - offset) / containerRef.value!.offsetWidth) * 10000
        ) / 100;
    }
    if (percent > 0 && percent < 100) {
      modelPercent.value = percent;
      hasMoved.value = true;
    }
  }

  function onBodyUp() {
    isActive.value = false;
    removeBodyListeners();
  }

  function removeBodyListeners() {
    document.body.removeEventListener("touchmove", onBodyTouchMove);
    document.body.removeEventListener("mousemove", onBodyMouseMove);
  }
</script>
