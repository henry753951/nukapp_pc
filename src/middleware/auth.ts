export default defineNuxtRouteMiddleware((to, from) => {
  const UserStore = useUserStore();
  const logger = useLogger("Auth Middleware");
  if (UserStore.user.isLoggedIn) {
    logger.info("User is logged in", "Passing through");
    return;
  }
  logger.info("User is not logged in", "Redirecting to unauthorized page");
  return navigateTo({
    name: "Unauthorized",
    query: {
      nextUrl: to.fullPath,
    },
  });
});
