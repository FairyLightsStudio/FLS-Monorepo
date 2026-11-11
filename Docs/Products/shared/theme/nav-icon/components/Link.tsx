import type { DocLink } from '../nav-config';

export const Link = ({
  name,
  desc,
  url,
  logo,
  logoScale,
  emoji,
  status,
}: DocLink) => {
  const icon = logo ? (
    <img
      className="fls-nav-link__logo"
      src={logo}
      alt={name}
      style={logoScale ? { transform: `scale(${logoScale})` } : undefined}
    />
  ) : (
    <span className="fls-nav-link__emoji" aria-hidden="true">
      {emoji ?? '📄'}
    </span>
  );

  const body = (
    <>
      {icon}
      <span className="fls-nav-link__content">
        <span className="fls-nav-link__name">{name}</span>
        <span className="fls-nav-link__desc">
          {status === 'soon' ? (
            <span className="fls-nav-link__badge">即将上线</span>
          ) : null}
          {desc}
        </span>
      </span>
    </>
  );

  if (status === 'ready' && url) {
    return (
      <a className="fls-nav-link" href={url} aria-label={`打开 ${name} 文档`}>
        {body}
      </a>
    );
  }

  return (
    <span className="fls-nav-link fls-nav-link--soon" aria-disabled="true">
      {body}
    </span>
  );
};
