import type { DocLink } from '../nav-config';
import { Link } from './Link';

export const LinkGroup = ({
  title,
  links,
}: {
  title?: string;
  links: DocLink[];
}) => (
  <div className="fls-nav-group">
    {title ? <div className="fls-nav-group__title">{title}</div> : null}
    <div className="fls-nav-group__links">
      {links.map((link) => (
        <Link key={link.name} {...link} />
      ))}
    </div>
  </div>
);
