import type { ToastMessageOptions } from "primevue/toast";

export const useNotification = () => {
  const notificationsStore = useNotificationsStore();
  const bus = useEventBus("notification");
  const onNotify = ref((event: unknown) => {});
  const unsubscribe = bus.on((event) => {
    onNotify.value(event);
  });
  onUnmounted(() => {
    unsubscribe();
  });

  const add = (notification: ToastMessageOptions) => {
    notificationsStore.add(notification);
    const bus = useEventBus("notification");
    bus.emit("notification", notification);
  };
  const pop = () => {
    return notificationsStore.pop();
  };
  const clear = () => {
    notificationsStore.clearNotifications();
  };

  return { add, pop, clear, onNotify };
};
