import React, { FC, HTMLAttributes, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { Album, Photo, fetchAlbumPhotos, selectOpenedAlbum } from '../../store/albums';
import PhotoGrid from '../PhotoGrid';
import AddPhoto from '../AddPhoto';
import PhotoComponent from './Photo';
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
          <PhotoComponent
            key={photo.id}
            id={photo.id}
            src={photo.src}
            mainColor={photo.mainColor}
            isFavorite={photo.isFavorite}
          />
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
