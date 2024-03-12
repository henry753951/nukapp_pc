<script lang="ts">
  import { VueMarkdownIt } from "@f3ve/vue-markdown-it";
  import { router } from "../router";
  export default {
    name: "QQ",
    components: {
      VueMarkdownIt,
    },
    data() {
      return {
        sourceUrl: "",
        content: "",
      };
    },
    async mounted() {
      this.sourceUrl = router.currentRoute.value.query.sourceURL as string;
      const response = await fetch(this.sourceUrl);
      this.content = await response.text();
    },
  };
</script>
<template>
  <div class="container">
    <vue-markdown-it class="md-body" :source="content" />
  </div>
</template>

<style>
  .container {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 0 50px;
  }
</style>
