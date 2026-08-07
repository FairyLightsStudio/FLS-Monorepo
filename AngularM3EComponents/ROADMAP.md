## 打算实现的feature

### 999 Storybook 交互测试与视觉测试方案决议

* **交互测试重构已落地**：已对 Storybook interaction tests（play 函数）进行了重构。实现了“静态展示 Story（仅作 A11y 静态断言，无 side effect 交互）”与“`Behavior/*` 交互测试 Story”的分离，彻底解决了 Storybook 预览界面被交互动作破坏并停留在非预期状态的问题。
* **解耦脆弱的 DOM/CSS 断言**：通过利用 A11y 标准属性（如 `aria-expanded`、`aria-selected`）及语义化可见性（`toBeVisible()`）代替了以往脆弱的 CSS 类名检测，成功缓解了 70% 的状态测试痛点。
* **视觉与排版测试移交商业化视觉测试**：对于其余 30% 的纯视觉/排版细节断言（例如指示器是 `hug` 还是 `fill`、垂直排列方式、是否有分割线等纯样式属性），由于高度绑定组件的私有 CSS/HTML 实现，编写常规断言非常脆弱且难以维护。我们决定**完全免除代码层面的样式断言**，未来直接引入 Storybook 商业化视觉测试服务（如 Chromatic）进行多分辨率视觉回归快照对比，彻底释放测试代码的维护开销。

## fix: 极窄屏设备下，bar可以触摸滚动items，rail在下有一些问题，suite派生出的rail、bar在极窄屏设备下有严重问题（滚动不了）

## 我已经注意到，但尚未打算实现的feature

### Navigation Suite 

#### 基于元素真实宽度的 rail 展开/收起过渡

当前 `navigation-suite` 为了让 expanded rail 收起时 main 面板跟随真实 rail 宽度平滑过渡，使用的是低频 JS 测量方案：在 rail 展开稳定后缓存实际宽度，收起前再冻结当前宽度作为 CSS transition 的起点。首次展开且尚无测量值时，main 面板先保持 collapsed rail offset，等 expanded rail 完全展开并测量到真实宽度后再让位，避免 grid 对绝对定位 rail surface 的 `max-content` 计算闪烁。

这不是最理想的实现方法。更理想的方向是纯 CSS intrinsic size 过渡，例如 `interpolate-size: allow-keywords` / `calc-size()`，让 `max-content` 与具体长度之间可以直接插值。等 Safari、Firefox 等浏览器支持后，应考虑移除 JS 测量兜底，改为纯 CSS 或以纯 CSS 为主的渐进增强实现。

#### 支持rail、bar外的自定义导航组件（例如 google keep 中的自定义 drawer）

目前我们已经抽象出并在rail、bar侧实现了通用的 `MatNavigationWidget` 接口与 `MAT_NAVIGATION_WIDGET` InjectionToken。

我们的目标是，任何第三方或自定义导航组件只需实现该接口并 provide 该 token，即可无缝嵌入 `mat-navigation-suite-scaffold` 中，实现物理高度与宽度的自动动态测量与自适应响应布局，不过我目前还没有第三方组件的需求，暂时不做这个。


### storybook 引入 compodoc，为文档页展示更全面的 API 列表

引入 compodoc 有问题。先把 没有compodoc的 storybook 搞定了再说。

### 尺寸与密度：Angular Material 官方可以为主题设定不同的、运行时改不了的Density

```scss
@use '@angular/material' as mat;

html {
  color-scheme: light dark;
  @include mat.theme(
    (
      color: mat.$violet-palette,
      typography: Roboto,
      density: 0,
      //这里能调节！
    )
  );
}
```

我不打算实现这个，因为我用不到！你要用的话请自行实现，能向我们发PR就最好啦。
