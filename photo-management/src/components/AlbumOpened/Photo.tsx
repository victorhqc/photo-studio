import React, { FC, HTMLAttributes, useState } from 'react';
import { XIcon } from '@primer/octicons-react';
import { PhotoColumn } from '../PhotoGrid';
import { Photo } from '../../store/albums';

const PhotoComponent: FC<Props> = ({
  id,
  mainColor,
  src,
  title,
  description,
  className,
  ...restOfProps
}) => {
  const [hover, setHover] = useState(false);

  return (
    <PhotoColumn
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      className={`album-photo__wrapper ${className || ''}`}
      key={id}
    >
      {hover && (
        <div className="album-photo__delete" title="Delete Photo">
          <XIcon size="medium" />
        </div>
      )}
      <div
        className="album-photo"
        style={{ backgroundColor: mainColor, backgroundImage: `url(${src})` }}
      />
      <p className="album-photo__title">{title}</p>
      <p className="album-photo__description">{description}</p>
    </PhotoColumn>
  );
};

type Props = HTMLAttributes<HTMLDivElement> &
  Pick<Photo, 'id' | 'mainColor' | 'src' | 'title' | 'description'>;

export default PhotoComponent;
