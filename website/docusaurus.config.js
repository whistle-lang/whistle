module.exports = {
  title: "Whistle",
  tagline: "One hella programming language",
  url: "https://whistle-land.github.io",
  baseUrl: "/",
  favicon: "assets/whistle_light.png",
  organizationName: "whistle-lang", // Usually your GitHub org/user name.
  projectName: "whistle", // Usually your repo name.
  themeConfig: {
    navbar: {
      title: "Whistle",
      logo: {
        alt: "Whistle Logo",
        src: "assets/whistle_dark.svg",
        srcDark: "assets/whistle_light.svg",
      },
      links: [
        {
          to: "docs/",
          activeBasePath: "docs",
          label: "Docs",
          position: "left",
        },
        { to: "blog", label: "Blog", position: "left" },
        {
          href: "https://github.com/whistle-lang/whistle",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Docs",
          items: [
            {
              label: "Introduction",
              to: "docs/",
            }
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "Discord",
              href: "https://discord.gg/hdKxd5x",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "Blog",
              to: "blog",
            },
            {
              label: "GitHub",
              href: "https://github.com/whistle-lang/whistle",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} the Whistle team.`,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          // It is recommended to set document id as docs home page (`docs/` path).
          homePageId: "whistle/introduction",
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          editUrl:
            "https://github.com/whistle-lang/whistle/edit/master/website/",
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            "https://github.com/whistle-lang/whistle/edit/master/website/blog/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
