import { Layout as BaseLayout } from '@rspress/core/theme-original';
import type { LayoutProps } from '@rspress/core/theme-original';
import { NavIcon } from './nav-icon';

/**
 * 所有产品文档站点共用的 Rspress 主题：
 * 只在默认 Layout 的导航标题前插入一个「产品切换」入口，其余完全沿用 Rspress 默认主题。
 */
const Layout = (props: LayoutProps) => (
  <BaseLayout {...props} beforeNavTitle={<NavIcon />} />
);

export * from '@rspress/core/theme-original';
export { Layout };
