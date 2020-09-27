import React, { FC, HTMLAttributes, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { Album, Photo, fetchAlbumPhotos, selectOpenedAlbum } from '../../store/albums';
import PhotoGrid, { PhotoColumn } from '../PhotoGrid';
import AddPhoto from '../AddPhoto';
import './styles.css';

const AlbumOpened: FC<Props> = ({ albumId, fetchAlbumPhotos, albumWithPhotos, ...restOfProps }) => {
  const [_album, photos] = albumWithPhotos || ([null, []] as [Album | null, Photo[]]);

  useEffect(() => {
    fetchAlbumPhotos(albumId);
  }, [albumId, fetchAlbumPhotos]);

  return (
    <div className="album-opened" {...restOfProps}>
      <PhotoGrid>
        {photos.map((photo) => (
          <PhotoColumn className="album-photo__wrapper" key={photo.id}>
            <div
              className="album-photo"
              style={{ backgroundColor: photo.mainColor, backgroundImage: `url(${photo.src})` }}
            />
            <p className="album-photo__title">{photo.title}</p>
            <p className="album-photo__description">{photo.description}</p>
          </PhotoColumn>
        ))}
      </PhotoGrid>
      <div className="album-opened__add">
        <AddPhoto />
      </div>
    </div>
  );
};

const mapStateToProps = (state: ApplicationState) => ({
  albumWithPhotos: selectOpenedAlbum(state),
});

const mapDispatchToProps = {
  fetchAlbumPhotos: fetchAlbumPhotos.request,
};

type Props = OwnProps &
  HTMLAttributes<HTMLDivElement> &
  ReturnType<typeof mapStateToProps> &
  typeof mapDispatchToProps;

type OwnProps = {
  albumId: string;
};

export default connect(mapStateToProps, mapDispatchToProps)(AlbumOpened);
