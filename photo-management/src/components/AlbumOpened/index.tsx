import React, { FC, HTMLAttributes, useEffect } from 'react';
import { connect } from 'react-redux';
import { Album, openAlbum, fetchAlbumPhotos } from '../../store/albums';
import AddPhoto from '../AddPhoto';
import './styles.css';

const AlbumOpened: FC<Props> = ({ data, fetchAlbumPhotos, openAlbum, ...restOfProps }) => {
  useEffect(() => {
    openAlbum(data);
    fetchAlbumPhotos(data.id);
  }, [fetchAlbumPhotos, openAlbum]);

  return (
    <div className="album-opened" {...restOfProps}>
      <AddPhoto />
    </div>
  );
};

const mapDispatchToProps = {
  openAlbum,
  fetchAlbumPhotos: fetchAlbumPhotos.request,
};

type Props = OwnProps & HTMLAttributes<HTMLDivElement> & typeof mapDispatchToProps;

type OwnProps = {
  data: Album;
};

export default connect(null, mapDispatchToProps)(AlbumOpened);
