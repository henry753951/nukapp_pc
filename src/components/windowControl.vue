<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";
import VueFeather from "vue-feather";

const appWindow = Window.getCurrent();
const { themeState, options } = useTheme();
const {
  locales,
  setLocale,
  locale: currentLocale,
  finalizePendingLocaleChange,
  t: $t,
} = useI18n();
const confirm = useConfirm();
const locale = computed({
  get: () => currentLocale.value,
  set: async (val) => {
    await setLocale(val);
    confirm.require({
      header: $t("translation.reload-header"),
      message: $t("translation.reload-message"),
      rejectProps: {
        label: $t("actions.cancel"),
        severity: "secondary",
        outlined: true,
      },
      acceptProps: {
        label: $t("actions.confirm"),
      },
      accept: () => {
        reloadNuxtApp({
          force: true,
          persistState: true,
        });
      },
      reject: () => {},
    });
  },
});

const localesOptions = computed(() => {
  return locales.value.map((locale) => {
    return {
      label: locale.language,
      value: locale.code,
    };
  });
});
</script>

<template>
  <div class="flex items-center" data-tauri-drag-region>
    <!--  -->
    <div class="pr-2">
      <Select
        v-model="themeState"
        :options="options"
        optionLabel="label"
        optionValue="value"
        min-w-8rem
      />
    </div>
    <div class="pr-2">
      <Select
        v-model="locale"
        :options="localesOptions"
        optionLabel="label"
        optionValue="value"
        min-w-8rem
      />
    </div>
    <!--  -->
    <div class="flex gap-1 my-auto ml-auto justify-center window-control">
      <div @click="appWindow.minimize()" class="box">
        <vue-feather type="minus" size="15"></vue-feather>
      </div>
      <div @click="appWindow.toggleMaximize()" class="box">
        <vue-feather type="square" size="15"></vue-feather>
      </div>
      <div @click="appWindow.close()" class="box">
        <vue-feather type="x" size="15"></vue-feather>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
* {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
.box {
  border-radius: 0.25rem;
  background-color: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
  padding: 5px;
  &:hover {
    background-color: rgba(255, 255, 255, 0.3);
  }
  &:last-child:hover {
    background-color: rgba(255, 0, 0, 0.4);
  }
}
</style>
