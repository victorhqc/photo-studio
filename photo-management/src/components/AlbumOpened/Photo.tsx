import React, { FC, HTMLAttributes, useState } from 'react';
import { connect } from 'react-redux';
import { XIcon } from '@primer/octicons-react';
import { PhotoColumn } from '../PhotoGrid';
import { Photo, deletePhoto } from '../../store/albums';

const PhotoComponent: FC<Props> = ({
  id,
  mainColor,
  src,
  title,
  description,
  className,
  deletePhoto,
  ...restOfProps
}) => {
  const [hover, setHover] = useState(false);

  return (
    <PhotoColumn
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      className={`album-photo__wrapper ${className || ''}`}
      key={id}
      {...restOfProps}
    >
      {hover && (
        <div
          className="album-photo__delete"
          title="Delete Photo"
          onClick={() => {
            deletePhoto(id);
          }}
        >
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

const mapDispatchToProps = {
  deletePhoto: deletePhoto.request,
};

type Props = HTMLAttributes<HTMLDivElement> &
  Pick<Photo, 'id' | 'mainColor' | 'src' | 'title' | 'description'> &
  typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(PhotoComponent);
