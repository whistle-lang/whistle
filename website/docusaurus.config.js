module.exports = {
  title: 'Whistle',
  tagline: 'One hella programming language',
  url: 'https://whistle-land.github.io',
  baseUrl: '/whistle/',
  favicon: 'img/favicon.ico',
  organizationName: 'whistle-land', // Usually your GitHub org/user name.
  projectName: 'whistle', // Usually your repo name.
  themeConfig: {
    navbar: {
      title: 'Whistle',
      logo: {
        alt: 'Whistle Logo',
        src: 'img/logo_light.svg',
        srcDark: 'img/logo_dark.svg'
      },
      links: [
        {
          to: 'docs/',
          activeBasePath: 'docs',
          label: 'Docs',
          position: 'left',
        },
        {to: 'blog', label: 'Blog', position: 'left'},
        {
          href: 'https://github.com/whistle-lang/whistle',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Style Guide',
              to: 'docs/',
            },
            {
              label: 'Second Doc',
              to: 'docs/doc2/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Stack Overflow',
              href: 'https://stackoverflow.com/questions/tagged/whistle',
            },
            {
              label: 'Discord',
              href: 'https://discordapp.com/invite/whistle',
            },
            {
              label: 'Twitter',
              href: 'https://twitter.com/whistle',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Blog',
              to: 'blog',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/whistle-lang/whistle',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} whistle team.`,
    },
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          // It is recommended to set document id as docs home page (`docs/` path).
          homePageId: 'doc1',
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          editUrl:
            'https://github.com/facebook/docusaurus/edit/master/website/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://github.com/facebook/docusaurus/edit/master/website/blog/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
