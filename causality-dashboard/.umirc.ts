import { defineConfig } from "@umijs/max";

export default defineConfig({
  extraPostCSSPlugins: [
    require("tailwindcss"),
    require("autoprefixer"),
  ],

  title: "Causality Front",
  layout: false,
  routes: [
    {
      routes: [
        {
          path: "/",
          redirect: "/home",
        },
        {
          path: "/home",
          component: "@/pages/home",
        },
        {
          path: "/vote",
          component: "@/pages/vote",
        },
        {
          path: "/alignment",
          component: "@/pages/alignment",
        },
        {
          path: "/subspace",
          component: "@/pages/subspace",
        },
        {
          path: "/governance/:subspaceId",
          component: "@/pages/governance",
        },
        {
          path: "/create-subspace",
          component: "@/pages/create-subspace",
        },
      ],
    },
  ],

  fastRefresh: true,
  chainWebpack(config) {
    config.module
      .rule("wasm")
      .test(/\.wasm$/)
      .use("file-loader")
      .loader("file-loader")
      .options({
        name: "static/wasm/[name].[hash:8].[ext]",
      });
  },
  hash: true,
});
