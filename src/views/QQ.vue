<script lang="ts">
  import { VueMarkdownIt } from "@f3ve/vue-markdown-it";
  export default {
    name: "QQ",
    components: {
      VueMarkdownIt,
    },
    data() {
      return {
        msg: "Welcome to Your Vue.js App",
        content: `
# 為什麼一代停用了
第一代版本因為使用了 PWA（Progressive Web App）技術，而PWA是一種網站應用程式。然而，由於瀏覽器的安全機制，它無法執行一些跨域請求（CORS）和處理Cookie，這導致了無法從瀏覽器本身進行網頁爬蟲操作的限制。

由於這些限制，我們無法直接從前端網頁中進行資料爬取，而需要透過後端伺服器來處理資料的抓取工作。因此，當用戶訪問或登入時，所有的請求都是通過後端伺服器進行，並且使用了我們的伺服器的IP地址進行訪問。

然而，由於一代版本中使用的IP地址被封鎖，我們無法繼續提供服務，因此第一代版本被停用了。在與學校進行協調之後，由於資安等因素，我們無法解決這些問題。

如果要繼續提供類似的功能，我們只能依靠本地端的應用程式來實現。然而，iOS應用程式的開發如果要上架，需要支付每年3000元的費用。由於我個人也是iOS使用者，因此僅開發Android應用程式因為個人私心，也不是一個解決方案 XD。

因此，在這幾天的開發中，我開發了桌面系統的應用程式，並且往後以開源的模式進行，這樣大家可以自行編譯，或者下載我們提供的發布版本。我們提供了Windows的發布版本，Linux MacOS 敬請期待。它目前僅提供模擬選課功能，日後韓會不斷增加其他功能。`,
      };
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
