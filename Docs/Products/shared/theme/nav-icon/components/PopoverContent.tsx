import type { NavConfig } from '../nav-config';
import { LinkGroup } from './LinkGroup';

export const PopoverContent = ({ config }: { config: NavConfig }) => (
  <div className="fls-nav-panel">
    <div className="fls-nav-panel__header">
      <span className="fls-nav-panel__title">彩灯工作室 · 文档</span>
      <a
        className="fls-nav-panel__repo"
        href="https://github.com/FairyLightsStudio/FLS-Monorepo"
        target="_blank"
        rel="noreferrer"
      >
        GitHub ↗
      </a>
    </div>
    {config.map((group, index) => (
      <LinkGroup
        key={group.title ?? index}
        title={group.title}
        links={group.links}
      />
    ))}
  </div>
);
