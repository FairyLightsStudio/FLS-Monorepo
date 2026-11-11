export type DocLink = {
  /** 产品名 */
  name: string;
  /** 中文描述 */
  desc: string;
  /** 英文描述（可选，当前站点为中文，保留字段方便以后做双语文档站） */
  descEn?: string;
  /** 已上线的文档地址；未上线留空 */
  url?: string;
  /** 图标（打包进主题的图片资源） */
  logo?: string;
  /** 个别图标需要缩放时使用 */
  logoScale?: number;
  /** 没有图片资源时用 emoji 兜底 */
  emoji?: string;
  status: 'ready' | 'soon';
};

export type NavGroup = {
  title?: string;
  links: DocLink[];
};

export type NavConfig = NavGroup[];

/**
 * 所有产品文档站共用的导航配置：新增产品时在对应分组里加一条即可。
 * 如果站点部署到子路径（例如 /terapanel/），把 url 改成对应地址；
 * 当前只有一个文档站点，先用 '/' 表示「本站」。
 */
export const navConfig: NavConfig = [
  {
    links: [
      {
        name: '泰拉面板',
        desc: '分布式服务器管理面板',
        descEn: 'Distributed server management panel',
        url: '/',
        logo: new URL('./assets/terapanel.webp', import.meta.url).href,
        status: 'ready',
      },
      {
        name: 'FairyLightsStudio',
        desc: '工作室官网',
        descEn: 'Studio website',
        emoji: '💞',
        status: 'soon',
      },
    ],
  },
];
