<template>
  <div absolute>
    <Toast position="top-left" group="tl" />
    <Toast position="top-center" group="tc" />
    <Toast position="top-right" group="tr" />
    <Toast position="bottom-left" group="bl" />
    <Toast position="bottom-center" group="bc" />
    <Toast position="bottom-right" group="br" />
  </div>
</template>

<script lang="ts" setup>
import Toast, { type ToastMessageOptions } from "primevue/toast";

const toast = useToast();
const logger = useLogger("MessageToast");

const { pop, onNotify } = useNotification();
const notificationsStore = useNotificationsStore();

// 當接收到通知時觸發
onNotify.value = (event) => {
  logger.info("收到通知", event);

  // 當通知存在時，將其一一顯示
  while (notificationsStore.notifications.length > 0) {
    const notification = pop();

    // 如果通知沒有指定 group，則預設為 "bc"
    if (notification && !notification.group) {
      notification.group = "bc";
    }

    // 將通知加入到 toast 顯示
    toast.add({ ...notification });
  }
};
</script>

<style></style>
