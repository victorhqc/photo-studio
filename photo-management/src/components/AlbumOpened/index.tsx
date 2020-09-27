import React, { FC, HTMLAttributes, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { Album, Photo, fetchAlbumPhotos, selectOpenedAlbum } from '../../store/albums';
import AddPhoto from '../AddPhoto';
import './styles.css';

const AlbumOpened: FC<Props> = ({ albumId, fetchAlbumPhotos, albumWithPhotos, ...restOfProps }) => {
  const [_album, photos] = albumWithPhotos || ([null, []] as [Album | null, Photo[]]);

  useEffect(() => {
    fetchAlbumPhotos(albumId);
  }, [albumId, fetchAlbumPhotos]);

  return (
    <div className="album-opened" {...restOfProps}>
      <div className="album-opened__photos">
        {photos.map((photo) => (
          <div className="album-photo__wrapper" key={photo.id}>
            <div
              className="album-photo"
              style={{ backgroundColor: photo.mainColor, backgroundImage: `url(${photo.src})` }}
            />
          </div>
        ))}
      </div>
      <AddPhoto />
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
