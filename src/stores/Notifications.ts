import { defineStore } from "pinia";
import type { ToastMessageOptions } from "primevue/toast";

export const useNotificationsStore = defineStore({
  id: "NotificationsStore",
  state: () => ({
    notifications: [] as ToastMessageOptions[],
  }),
  actions: {
    add(notification: ToastMessageOptions) {
      this.notifications.push(notification);
    },
    pop() {
      return this.notifications.pop();
    },
    clearNotifications() {
      this.notifications = [];
    },
  },
});
