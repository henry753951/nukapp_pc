import { theme } from "ant-design-vue";

export const useTheme = () => {
  const { $i18n } = useNuxtApp();
  const tokens = useState("themeToken", () => ({
    token: {
      colorBgLayout: "var(--colorBgLayout)",
      colorPrimaryBg: "var(--colorPrimaryBg)",
      colorPrimary: "var(--colorPrimary)",
      colorSideBg: "var(--colorSideBg)",
    },
  }));

  const themeData = computed({
    get: () => {
      return {
        algorithm:
          currentTheme.value != "dark"
            ? theme.defaultAlgorithm
            : theme.darkAlgorithm,
        ...tokens.value,
      };
    },
    set: (val) => {
      const { algorithm, ...tokens_ } = val;
      tokens.value = tokens_;
    },
  });

  const isPreferredDark = useMediaQuery("(prefers-color-scheme: dark)");
  const themeStore = useThemeStore();
  const themeState = computed({
    get: () => themeStore.theme,
    set: (val) => {
      themeStore.theme = val;
    },
  });
  const currentTheme = computed(() => {
    if (themeState.value === "default") {
      return isPreferredDark.value ? "dark" : "light";
    }
    return themeState.value;
  });

  const options = [
    { value: "default", label: $i18n.t("theme.default") },
    { value: "dark", label: $i18n.t("theme.dark") },
    { value: "light", label: $i18n.t("theme.light") },
  ];

  return { themeState, currentTheme, options, themeData };
};
