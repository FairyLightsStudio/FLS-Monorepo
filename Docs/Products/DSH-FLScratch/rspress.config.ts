import * as path from 'node:path';
import { defineConfig } from '@rspress/core';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  // 所有产品站点共用同一套主题（导航栏产品切换入口）
  themeDir: path.join(__dirname, '..', 'shared', 'theme'),
  title: 'DSH-FLScratch',
  icon: '/DSHFLScratchIcon.webp',
  logo: {
    light: '/DSHFLScratchLogo-light.webp',
    dark: '/DSHFLScratchLogo-dark.webp',
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
