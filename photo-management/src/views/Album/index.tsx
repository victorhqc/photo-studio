import React, { FC, ComponentType, useEffect } from 'react';
import { compose } from 'redux';
import { connect } from 'react-redux';
import { withRouter, RouteComponentProps } from 'react-router';
import { ApplicationState } from '../../store';
import { openAlbum, fetchAllAlbums, selectAlbumById, selectAlbums } from '../../store/albums';
import AlbumOpened from '../../components/AlbumOpened';
import './styles.css';

const AlbumView: FC<Props> = ({ openAlbum, fetchAllAlbums, match, album, albumsLength }) => {
  const {
    params: { id },
  } = match;

  useEffect(() => {
    if (!album) {
      return;
    }

    openAlbum(album[0]);
  }, [id, openAlbum, album]);

  useEffect(() => {
    if (albumsLength === 0) {
      fetchAllAlbums();
    }
  }, [fetchAllAlbums, albumsLength]);

  if (!album) {
    return null;
  }

  return (
    <div className="home__default-album">
      <AlbumOpened albumId={album[0].id} />
    </div>
  );
};

const mapStateToProps = (
  state: ApplicationState,
  { match }: RouteComponentProps<{ id: string }>
) => ({
  album: selectAlbumById(state, match.params.id),
  albumsLength: selectAlbums(state).length,
});

const mapDispatchToProps = {
  fetchAllAlbums: fetchAllAlbums.request,
  openAlbum,
};

type Props = ReturnType<typeof mapStateToProps> &
  typeof mapDispatchToProps &
  RouteComponentProps<{ id: string }>;

export default compose<ComponentType>(
  withRouter,
  connect(mapStateToProps, mapDispatchToProps)
)(AlbumView);
