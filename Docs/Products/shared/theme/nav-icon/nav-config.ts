import terapanelLogo from './assets/terapanel.webp';

export type DocProduct = {
  id: string;
  name: string;
  desc: string;
  /** 已上线的文档站点地址；未上线留空 */
  url?: string;
  /** 图标（打包进主题的图片资源） */
  logo?: string;
  /** 没有图片资源时用 emoji 兜底 */
  emoji?: string;
  status: 'ready' | 'soon';
};

/**
 * 产品文档导航配置：新增产品时在这里加一条即可。
 * 注意：如果站点部署在子路径（例如 /terapanel/），把 url 改成对应地址；
 * 当前只有一个站点，先用 '/' 表示「本站」。
 */
export const products: DocProduct[] = [
  {
    id: 'terapanel',
    name: '泰拉面板',
    desc: '分布式服务器管理面板',
    url: '/',
    logo: terapanelLogo,
    status: 'ready',
  },
  {
    id: 'aboutus',
    name: 'FairyLightsStudio',
    desc: '工作室官网',
    emoji: '💞',
    status: 'soon',
  },
];
