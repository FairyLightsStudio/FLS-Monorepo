import Popover from 'antd/es/popover';
import { PopoverContent } from './components/PopoverContent';
import { navConfig } from './nav-config';
import './index.css';

export type NavIconProps = {
  trigger?: 'hover' | 'focus' | 'click' | Array<'hover' | 'focus' | 'click'>;
};

/**
 * 导航栏左侧的产品切换入口。
 * 实现方式参考 Rstack 文档站的 @rstack-dev/doc-ui NavIcon：
 * antd Popover（bottomLeft、无箭头）+ 产品列表；这里换成我们自己的 nav-config。
 */
export const NavIcon = ({ trigger = 'hover' }: NavIconProps = {}) => (
  <div className="fls-nav-icon">
    <Popover
      arrow={false}
      trigger={trigger}
      placement="bottomLeft"
      content={<PopoverContent config={navConfig} />}
      classNames={{ root: 'fls-nav-popover' }}
      styles={{
        root: {
          padding: 0,
          background: 'transparent',
          boxShadow: 'none',
          zIndex: 999,
        },
        container: {
          padding: 0,
          background: 'transparent',
          boxShadow: 'none',
        },
      }}
    >
      <button
        type="button"
        className="fls-nav-icon__wrapper"
        aria-label="切换产品文档"
      >
        <svg
          width="20"
          height="20"
          viewBox="0 0 20 20"
          fill="none"
          aria-hidden="true"
        >
          <path
            d="M2.5 5H5V2.5H2.5V5zM5 11.25H2.5v-2.5H5v2.5zm12.5 0H15v-2.5h2.5v2.5zm-8.75 0h2.5v-2.5h-2.5v2.5zM5 17.5H2.5V15H5v2.5zm10 0h2.5V15H15v2.5zm-3.75 0h-2.5V15h2.5v2.5zM17.5 5H15V2.5h2.5V5zM8.75 5h2.5V2.5h-2.5V5z"
            fill="currentColor"
          />
        </svg>
      </button>
    </Popover>
  </div>
);
