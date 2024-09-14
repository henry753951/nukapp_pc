import { Window } from "@tauri-apps/api/window";

const initFocus = () => {
  const { currentTheme } = useTheme();
  const GlobalState = useGlobalStateStore();
  const isFocused = computed({
    get: () => GlobalState.window.isFocused,
    set: (value) => {
      GlobalState.window.isFocused = value;
    },
  });

  const onFocus = (focus: boolean) => {
    isFocused.value = focus;
    if (!focus) {
      document.documentElement.style.setProperty(
        "background-color",
        "var(--unfocus-surface)"
      );
    } else {
      document.documentElement.style.removeProperty("background-color");
    }
  };
  const appWindow = Window.getCurrent();
  appWindow.listen("tauri://focus", () => onFocus(true));
  appWindow.listen("tauri://blur", () => onFocus(false));
};

const initTheme = () => {
  const { themeData, currentTheme } = useTheme();
  const setClass = (val: string) => {
    document.documentElement.classList.toggle("dark", val === "dark");
    document.documentElement.classList.toggle("light", val === "light");
  };
  setClass(currentTheme.value);
  watch(currentTheme, (val) => {
    setClass(val);
  });
};

export default defineNuxtPlugin(() => {
  if (import.meta.client) {
    initFocus();
    initTheme();
  }
});
