import { useEffect, useRef, useState } from 'react';
import type { DocProduct } from './nav-config';
import { products } from './nav-config';
import './index.css';

const GridIcon = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 20 20"
    fill="none"
    aria-hidden="true"
  >
    {[2, 8, 14].map((x) =>
      [2, 8, 14].map((y) => (
        <rect
          key={`${x}-${y}`}
          x={x}
          y={y}
          width="4"
          height="4"
          rx="1"
          fill="currentColor"
        />
      )),
    )}
  </svg>
);

const ProductItem = ({ product }: { product: DocProduct }) => {
  const icon = product.logo ? (
    <img
      className="fls-nav-icon__logo"
      src={product.logo}
      alt={product.name}
    />
  ) : (
    <span className="fls-nav-icon__emoji" aria-hidden="true">
      {product.emoji ?? '📄'}
    </span>
  );

  const body = (
    <>
      {icon}
      <span className="fls-nav-icon__content">
        <span className="fls-nav-icon__name">
          {product.name}
          {product.status === 'soon' ? (
            <span className="fls-nav-icon__badge">即将上线</span>
          ) : null}
        </span>
        <span className="fls-nav-icon__desc">{product.desc}</span>
      </span>
    </>
  );

  if (product.status === 'ready' && product.url) {
    return (
      <a
        className="fls-nav-icon__item"
        href={product.url}
        role="menuitem"
        aria-label={`打开 ${product.name} 文档`}
      >
        {body}
      </a>
    );
  }

  return (
    <span
      className="fls-nav-icon__item fls-nav-icon__item--soon"
      aria-disabled="true"
    >
      {body}
    </span>
  );
};

/**
 * 导航栏左侧的「产品文档」切换入口。
 * 交互：鼠标悬停展开；点击按钮切换；Escape 或点击外部关闭。
 */
export const NavIcon = () => {
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);
  const closeTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  useEffect(() => {
    if (!open) {
      return;
    }
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setOpen(false);
      }
    };
    const onPointerDown = (event: MouseEvent) => {
      if (rootRef.current && !rootRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener('keydown', onKeyDown);
    document.addEventListener('mousedown', onPointerDown);
    return () => {
      document.removeEventListener('keydown', onKeyDown);
      document.removeEventListener('mousedown', onPointerDown);
    };
  }, [open]);

  useEffect(
    () => () => {
      if (closeTimer.current) {
        clearTimeout(closeTimer.current);
      }
    },
    [],
  );

  const cancelClose = () => {
    if (closeTimer.current) {
      clearTimeout(closeTimer.current);
      closeTimer.current = undefined;
    }
  };
  const scheduleClose = () => {
    cancelClose();
    closeTimer.current = setTimeout(() => setOpen(false), 150);
  };

  return (
    <div
      className="fls-nav-icon"
      ref={rootRef}
      onMouseEnter={() => {
        cancelClose();
        setOpen(true);
      }}
      onMouseLeave={scheduleClose}
    >
      <button
        type="button"
        className="fls-nav-icon__button"
        aria-label="切换产品文档"
        aria-expanded={open}
        aria-haspopup="menu"
        onClick={() => setOpen(true)}
      >
        <GridIcon />
      </button>
      {open ? (
        <div className="fls-nav-icon__panel" role="menu">
          <div className="fls-nav-icon__header">
            <span className="fls-nav-icon__brand">彩灯工作室 · 文档</span>
            <a
              className="fls-nav-icon__repo"
              href="https://github.com/FairyLightsStudio/FLS-Monorepo"
              target="_blank"
              rel="noreferrer"
            >
              GitHub ↗
            </a>
          </div>
          <div className="fls-nav-icon__grid">
            {products.map((product) => (
              <ProductItem key={product.id} product={product} />
            ))}
          </div>
        </div>
      ) : null}
    </div>
  );
};
