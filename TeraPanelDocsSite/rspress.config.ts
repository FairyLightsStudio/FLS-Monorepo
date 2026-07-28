import * as path from 'node:path';
import { defineConfig } from '@rspress/core';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  title: '泰拉面板',
  icon: '/TeraPanelIcon.webp',
  logo: {
    light: '/TeraPanelLogo-light.webp',
    dark: '/TeraPanelLogo-dark.webp',
  },
  themeConfig: {
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: 'https://github.com/web-infra-dev/rspress',
      },
    ],
  },
});
