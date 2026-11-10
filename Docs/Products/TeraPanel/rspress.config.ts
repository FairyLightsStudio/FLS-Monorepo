import * as path from 'node:path';
import { defineConfig } from '@rspress/core';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  // 所有产品站点共用同一套主题（导航栏产品切换入口）
  themeDir: path.join(__dirname, '..', 'shared', 'theme'),
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
        content: 'https://github.com/FairyLightsStudio/FLS-Monorepo',
      },
    ],
  },
});
