import { Directive, ContentChild, input } from '@angular/core';
import { MatNavigationIcon, MatNavigationActiveIcon, MatNavigationLabel } from './directives';

@Directive()
export class MatNavigationItemBase {
  /** Whether the navigation item is active/selected. */
  active = input<boolean>(false);

  /** Content query for the default icon. */
  @ContentChild(MatNavigationIcon) protected icon?: MatNavigationIcon;

  /** Content query for the active/selected icon. */
  @ContentChild(MatNavigationActiveIcon) protected activeIcon?: MatNavigationActiveIcon;

  /** Content query for the label. */
  @ContentChild(MatNavigationLabel) protected label?: MatNavigationLabel;
}
