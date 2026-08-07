# Navigation Suite

Responsive navigation suite scaffold with automatic breakpoint switching between navigation-bar and navigation-rail for Angular Material.

> This is a **third-party** implementation by FairyLights Studio, not an official Angular / Google component.

## Visibility state

`MatNavigationSuiteScaffoldState` controls navigation visibility:

```ts
scaffoldState = new MatNavigationSuiteScaffoldState('visible');

await scaffoldState.hide();
await scaffoldState.show();
await scaffoldState.toggle();
await scaffoldState.snapTo('hidden');
```

The state exposes `currentValue`, `targetValue`, and `isAnimating` signals. The Promise returned by `show()`, `hide()`, `toggle()`, and `snapTo()` resolves when the scaffold reaches the settled state. In bar layouts, `matNavigationSuitePrimaryAction` remains visible when the navigation bar hides; in rail layouts, it remains part of the rail header and hides with the rail. Rail hide/show mirrors Angular Material sidenav: the rail surface keeps its own width and slides with `translate3d()`, while the content area expands or reserves space separately.

[查看源代码 / Source](https://tangled.org/fairylights.org/AngularM3EComponents.git) · [查看文档 / Docs](https://some-angular-m3e-components.pages.dev/?path=/docs/navigation-navigation-suite-scaffold--docs)
