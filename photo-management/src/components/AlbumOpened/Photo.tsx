import React, { FC, HTMLAttributes, useState, useCallback } from 'react';
import { connect } from 'react-redux';
import contrast from 'contrast';
import { XIcon, StarIcon, StarFillIcon } from '@primer/octicons-react';
import { PhotoColumn } from '../PhotoGrid';
import { Photo, deletePhoto, updatePhoto } from '../../store/albums';

const PhotoComponent: FC<Props> = ({
  id,
  mainColor,
  src,
  isFavorite,
  className,
  deletePhoto,
  updatePhoto,
  ...restOfProps
}) => {
  const [hover, setHover] = useState(false);
  const isDark = contrast(mainColor) === 'dark';

  const handleToggleFavorite = useCallback(() => {
    updatePhoto({
      id,
      isFavorite: !isFavorite,
      title: null,
      description: null,
    });
  }, [id, isFavorite, updatePhoto]);

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
          style={{ color: isDark ? 'white' : 'black' }}
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
      >
        <button
          className="album-photo__is-favorite"
          style={{ color: isDark ? 'white' : 'black' }}
          onClick={handleToggleFavorite}
          title="favorite"
        >
          {isFavorite && <StarFillIcon size="medium" />}
          {!isFavorite && <StarIcon size="medium" />}
        </button>
      </div>
    </PhotoColumn>
  );
};

const mapDispatchToProps = {
  deletePhoto: deletePhoto.request,
  updatePhoto: updatePhoto.request,
};

type Props = HTMLAttributes<HTMLDivElement> &
  Pick<Photo, 'id' | 'mainColor' | 'src' | 'isFavorite'> &
  typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(PhotoComponent);
